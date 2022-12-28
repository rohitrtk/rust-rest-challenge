# Challenge Details

A client asks you to create a REST client to handle their data, which consists of various movie actors, the movies they have starred in, and other information about them.

For example, Mark Hamill is a 71 year old male, born on September 25<sup>th</sup>, 1951, who starred in Star Wars IV, V, VI, VII, and VIII.

# Routes

### GET Requests

- <b>GET</b> /api/getActors
  - Description: Returns a list of all actors in JSON format.
  - Response Body: 
    ```
    {
      "id": 1
      "first_name": "Mark",
      "last_name": "Hamill",
      "age": 71,
      "dob": "09/25/1951",
      "movies": [
        "Star Wars IV",
        "Star Wars V",
        "Star Wars VI",
        "Star Wars VII",
        "Star Wars VIII",
        ...
      ]
    },
    {
      "id": 2
      "first_name": "Harrison",
      "last_name": "Ford",
      ...
    },
    ...
    ```
  - Expected Responses:
    - 200 OK - Successful response
    - 500 BAD REQUEST - If getting the information was unsuccessful
- <b>GET</b> /api/getActor
  - Description: Checks if the specified actor by ID exists.
  - Request Body:
    ```
    {
      "id": 1
    }
    ```
  - Response Body:
    ```
    {
      "id": 1
      "first_name": "Mark",
      "last_name": "Hamill",
      "age": 71,
      "dob": "09/25/1951",
      "movies": [
        "Star Wars IV",
        "Star Wars V",
        "Star Wars VI",
        "Star Wars VII",
        "Star Wars VIII",
        ...
      ]
    }
    ```
    - Expected Responses:
      - 200 OK - Sucessfully retrived actor information
      - 400 BAD REQUEST - If the request body was formatted incorrectly or missing the required information
      - 404 NOT FOUND - If there is no actor with the specified id
      - 500 INTERNAL SERVER ERROR - If getting the information was unsuccessful

### POST Requests

- <b>POST</b> /api/addActor
    - Description: Adds an actor to the list of actors
    - Request Body:
        ```
        {
          "first_name": "Carrie",
          "last_name": "Fisher",
          "age": 60,
          "dob": "10/21/1956",
          "movies": [
            "Star Wars IV",
            "Star Wars V",
            "Star Wars VI"
          ]
        }
        ```
    - Response Body:
      ```
      {
        "id": 3,
        "first_name": "Carrie",
        "last_name": "Fisher",
        "age": 60,
        "dob": "10/21/1956",
        "movies": [
          "Star Wars IV",
          "Star Wars V",
        ]
      }
      ```
    - Expected Responses:
      - 200 OK - If the actor was successfully added
      - 400 BAD REQUEST - If the request body was formatted incorrectly or missing the required information
      - 500 INTERNAL SERVER ERROR - If getting the information was unsuccessful

### PUT Requests

- <b>PUT</b> /api/updateActor
  - Description: Updates the specified actors information
  - Request Body:
    ```
    {
      "id": 3,
      "movies": [
        "Star Wars IV",
        "Star Wars V",
        "Star Wars VI"
      ]
    }
    ```
  - Response Body:
    ```
    {
      "id": 3,
      "first_name": "Carrie",
      "last_name": "Fisher",
      "age": 60,
      "dob": "10/21/1956",
      "movies": [
        "Star Wars IV",
        "Star Wars V",
        "Star Wars VI"
      ]
    }
    ```
  - Expected Responses:
    - 200 OK - If updating the actors information was successul
    - 400 BAD REQUEST - If the request body was formatted incorrectly or missing the required information
    - 404 NOT FOUND - If the requested actor id does not exist
    - 500 INTERNAL SERVER ERROR - If getting the information was unsuccessful

### DELETE Requests

- <b>DELETE</b> /api/deleteActor
  - Description: Deletes the specified actor
  - Request Body:
    ```
    {
      "id": 3
    }
    ```
  - Expected Responses:
    - 200 OK - If the the actor was successfully deleted
    - 400 BAD REQUEST - If the request body was formatted incorrectly or missing the required information
    - 404 NOT FOUND - If the requested actor id does not exist
    - 500 INTERNAL SERVER ERROR - If getting the information was unsuccessful