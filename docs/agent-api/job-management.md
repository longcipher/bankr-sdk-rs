On this page

After submitting a prompt, use these endpoints to track and manage your jobs.

## Get Job Status​


    GET /agent/job/{jobId}  


### Headers​

Header| Value| Required  
---|---|---  
`X-API-Key`| Your API key| Yes  

### Path Parameters​

Parameter| Type| Description  
---|---|---  
`jobId`| string| Job identifier from submit prompt response  

### Response (200 OK)​


    {  
      "success": true,  
      "jobId": "abc123",  
      "threadId": "thr_XYZ789",  
      "status": "completed",  
      "prompt": "what is the price of ETH?",  
      "createdAt": "2024-01-15T10:30:00Z",  
      "completedAt": "2024-01-15T10:30:03Z",  
      "processingTime": 3000,  
      "response": "ETH is currently trading at $3,245.67",  
      "richData": [],  
      "cancellable": false  
    }  


### Response Fields​

Field| Type| Description| When Present  
---|---|---|---  
`success`| boolean| Whether the request succeeded| Always  
`jobId`| string| Job identifier| Always  
`threadId`| string| Conversation thread ID| When set  
`status`| JobStatus| Current job status| Always  
`prompt`| string| Original prompt submitted| Always  
`createdAt`| string| ISO 8601 timestamp| Always  
`cancellable`| boolean| Whether job can be cancelled| pending/processing  
`statusUpdates`| StatusUpdate[]| Progress messages| When available  
`startedAt`| string| When processing started| processing  
`response`| string| AI agent's response| completed  
`richData`| RichData[]| Additional structured data| completed  
`completedAt`| string| When job finished| completed/failed  
`processingTime`| number| Duration in milliseconds| completed  
`error`| string| Error message| failed  
`cancelledAt`| string| When job was cancelled| cancelled  

### Job Status Values​

Status| Description  
---|---  
`pending`| Job is queued for processing  
`processing`| Job is currently being processed  
`completed`| Job finished successfully  
`failed`| Job encountered an error  
`cancelled`| Job was cancelled by user  

### Error Responses​

**Job ID Required (400)**

    {  
      "error": "Job ID required",  
      "message": "Please provide a job ID"  
    }  

**Job Not Found (404)**

    {  
      "error": "Job not found",  
      "message": "No job found with ID abc123"  
    }  


## Cancel Job​


    POST /agent/job/{jobId}/cancel  

Cancel a pending or processing job.

### Headers​

Header| Value| Required  
---|---|---  
`X-API-Key`| Your API key| Yes  

### Path Parameters​

Parameter| Type| Description  
---|---|---  
`jobId`| string| Job identifier to cancel  

### Response (200 OK)​


    {  
      "success": true,  
      "jobId": "abc123",  
      "status": "cancelled",  
      "prompt": "swap $100 ETH to USDC",  
      "createdAt": "2024-01-15T10:30:00Z",  
      "cancelledAt": "2024-01-15T10:30:05Z"  
    }  

note

Cancel requests are idempotent. Cancelling an already-cancelled job returns success.

### Error Responses​

**Job Already Completed (400)**

    {  
      "success": false,  
      "error": "Job already completed",  
      "message": "Cannot cancel a completed job"  
    }  

**Job Already Failed (400)**

    {  
      "success": false,  
      "error": "Job already failed",  
      "message": "Cannot cancel a failed job"  
    }  


## Polling Strategy​

### Recommended Approach​


    async function pollForCompletion(jobId: string, maxAttempts = 60) {  
      const API_KEY = process.env.BANKR_API_KEY;  

      for (let i = 0; i < maxAttempts; i++) {  
        const response = await fetch(  
          `https://api.bankr.bot/agent/job/${jobId}`,  
          {  
            headers: { 'X-API-Key': API_KEY },  
          }  
        );  

        if (!response.ok) {  
          throw new Error('Failed to fetch job status');  
        }  

        const job = await response.json();  
        console.log(`Status: ${job.status}`);  

        if (job.status === 'completed') {  
          return {  
            response: job.response,  
            richData: job.richData,  
          };  
        }  

        if (job.status === 'failed') {  
          throw new Error(job.error || 'Job failed');  
        }  

        if (job.status === 'cancelled') {  
          throw new Error('Job was cancelled');  
        }  

        // Wait 2 seconds before next poll  
        await new Promise((resolve) => setTimeout(resolve, 2000));  
      }  

      throw new Error('Timeout waiting for job completion');  
    }  


### Polling Parameters​

Parameter| Recommended| Notes  
---|---|---  
Interval| 2 seconds| Balance between responsiveness and API load  
Max attempts| 60| ~2 minutes total timeout  
Total timeout| 2-5 minutes| Most jobs complete within 30 seconds  

## Complete Example​


    const API_BASE_URL = "https://api.bankr.bot/agent";  
    const API_KEY = process.env.BANKR_API_KEY!;  

    async function executePrompt(prompt: string) {  
      // 1) Submit the prompt  
      const submitResponse = await fetch(`${API_BASE_URL}/prompt`, {  
        method: "POST",  
        headers: {  
          "Content-Type": "application/json",  
          "X-API-Key": API_KEY,  
        },  
        body: JSON.stringify({ prompt }),  
      });  

      if (!submitResponse.ok) {  
        const error = await submitResponse.json();  
        throw new Error(error.message || "Failed to submit prompt");  
      }  

      const { jobId } = await submitResponse.json();  
      console.log(`Job submitted: ${jobId}`);  

      // 2) Poll for completion  
      const result = await pollForCompletion(jobId);  
      return result;  
    }  

    async function pollForCompletion(jobId: string, maxAttempts = 60) {  
      for (let i = 0; i < maxAttempts; i++) {  
        const statusResponse = await fetch(`${API_BASE_URL}/job/${jobId}`, {  
          headers: { "X-API-Key": API_KEY },  
        });  

        if (!statusResponse.ok) {  
          throw new Error("Failed to fetch job status");  
        }  

        const job = await statusResponse.json();  
        console.log(`Status: ${job.status}`);  

        if (job.status === "completed") {  
          return {  
            response: job.response,  
            richData: job.richData,  
          };  
        }  

        if (job.status === "failed") {  
          throw new Error(job.error || "Job failed");  
        }  

        if (job.status === "cancelled") {  
          throw new Error("Job was cancelled");  
        }  

        // Wait 2 seconds before next poll  
        await new Promise((resolve) => setTimeout(resolve, 2000));  
      }  

      throw new Error("Timeout waiting for job completion");  
    }  

    async function cancelJob(jobId: string) {  
      const response = await fetch(`${API_BASE_URL}/job/${jobId}/cancel`, {  
        method: "POST",  
        headers: { "X-API-Key": API_KEY },  
      });  

      if (!response.ok) {  
        const error = await response.json();  
        throw new Error(error.message || "Failed to cancel job");  
      }  

      return response.json();  
    }  

    // Usage  
    async function main() {  
      const result = await executePrompt("What is the current price of ETH?");  
      console.log("Response:", result.response);  
      console.log("Rich Data:", result.richData);  
    }  

    main().catch(console.error);  


## Rich Data​

Completed jobs may include `richData` with additional structured information:

    {  
      "richData": [  
        {  
          "type": "token_info",  
          "symbol": "ETH",  
          "price": 3245.67,  
          "change24h": 2.3  
        },  
        {  
          "type": "chart",  
          "url": "https://..."  
        }  
      ]  
    }  

The structure varies based on the type of query and response.
