Feature: User Asset Information Retrieval
  As a client,
  I want to retrieve user asset information,
  So that I can provide users with their asset details

  Scenario: Retrieve user assets with valid details
    Given I have a valid access token
    When I send a GET request to "/v5/assets"
    Then The response should be 200 code
    Then I should receive user assets information
