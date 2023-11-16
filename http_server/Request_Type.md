# Http Request Types

HTTP (Hypertext Transfer Protocol) defines a set of request methods or HTTP methods that are used to indicate the desired action to be performed on a resource. 

HTTP methods are an essential part of the HTTP protocol and play a crucial role in defining how clients and servers interact with resources on the web. Each method has its specific purpose and behavior, and understanding when and how to use them is important for building effective and secure web applications.

Before building a server of any kind, you should know the 9 methods. See below:


1. **GET:** The GET method requests data from a specified resource. It is used to retrieve information from the server, and it should not have any side effects on the server or modify the resource.

2. **POST:** The POST method submits data to be processed to a specified resource. It is commonly used for creating new resources on the server, such as submitting form data or uploading files.

3. **PUT:** The PUT method updates or replaces a resource at a specified URI with the provided representation. It is idempotent, meaning that repeated PUT requests have the same effect as a single request.

4. **PATCH:** The PATCH method applies partial modifications to a resource. It is typically used when you want to update a portion of a resource's data without affecting the rest of it. Like PUT, it is idempotent.

5. **DELETE:** The DELETE method requests the removal of a resource at a specified URI. It is used to delete a resource on the server. Repeated DELETE requests on the same resource will result in the resource being deleted only once.

6. **HEAD:** The HEAD method is similar to GET but requests only the headers of a resource, not the actual data. It is often used to check for the existence or status of a resource without transferring the full content.

7. **OPTIONS:** The OPTIONS method is used to describe the communication options for the target resource. It is commonly used for discovering available methods, server capabilities, or CORS (Cross-Origin Resource Sharing) information.

8. **CONNECT**: The CONNECT method is used to establish a network connection to a resource, typically for proxying requests through a gateway or tunnel.

9. **TRACE:** The TRACE method is used to retrieve a diagnostic trace of the actions performed by the server on the received request. It is primarily used for debugging and should not have significant security implications.

