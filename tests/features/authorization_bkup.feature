Feature: User Asset Information Retrieval
  As a client,
  I want to retrieve user asset information,
  So that I can provide users with their asset details

  Scenario: Retrieve user assets with valid details
    Given I have a valid access token
    When I send a GET request to "/v5/assets"
    Then The response should be 200 code
    Then I should receive user assets information

  Scenario: Retrieve user assets with invalid token
    Given I have an invalid access token
    When I send a GET request to "/v5/assets"
    Then The response should be 401 code
    Then I should receive an error message indicating the invalid/expired token

  Scenario: Retrieve user assets with a missing required header
    Given I have a valid access token
    Given I am missing a required header in my request
    When I send a GET request to "/v5/assets"
    Then The response should be 400 code
    Then I should receive an error message indicating the missing header

  Scenario: Retrieve user assets without a valid TPP session
    Given I have a valid access token
    Given I do not have a valid TPP session
    When I send a GET request to "/v5/assets"
    Then The response should be 403 code
    Then I should receive an error message indicating the lack of a valid TPP session

  Scenario: Retrieve user assets after PSU session expires
    Given I have a valid access token
    Given the PSU session has just expired
    When I send a GET request to "/v5/assets"
    Then The response should be 401 code
    Then I should receive an error message indicating the expired PSU session