# nope.rs

## No as a Service.
> A simple, localizable API that says “no” in many ways, across many languages.



Use it for task like:

- Denying access with flair
- Fun error messaging
- Saying “no” (but with pazazz)


# Usage

Main endpoint: `https://nope.rs/nope`

> Return structure
```ts
{
  "success": true, // Boolean indicating whether or not the request was successfully completed internally.
  "data": { // Object containing the nope data
    "language": "en", // iso language code
    "nope": "I’d consider it… if the laws of physics didn’t apply." // Actual nope phrase.
  },
  "error": null, // If there was an error, this will contain the error message.
  "message": "Successfully got random nope." // User friendly response message.
}
```

> Parameters

Just tack on a query parameter `?lang=code` to choose one of the supported languages. Otherwise, it defaults to English.

> Response codes.

nope.rs returns 200 or 429. Right now I have no need for admin routes so you will never see a 403. 

429 does not follow the same response structure and is simple text explaining how long until you can try again. Read rate limits from response status code please.

# Supported Languages

I support mostly english, but am looking for as many languages as people wish to PR in.



##  Self-hosting
Just run the dockerfile.


> Environment options:

```
PORT=8080 (Whatever port you want the api to run on)

Note, the below keys are remnants from my api base but remain in the case I choose to add runtime list updates.
API_KEY_0="full access api key"
API_KEY_1="controlled access read write api key"
API_KEY_2="read only api key"

```

# Credits
### As people contribute they will be listed here.  

[Hotheadhacker](https://github.com/hotheadhacker) for making the original no as a service api which inspired this. 

[Err0r430](https://x.com/itsnoahd) Making this. 


# License

This is a "Do what you want" license.

You may use, modify, and distribute this project however you like.
The only requirement: **credit me somewhere.**
