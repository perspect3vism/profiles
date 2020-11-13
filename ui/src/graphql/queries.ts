import { gql } from '@apollo/client/core';

export const CREATE_PROFILE = gql`
  mutation CreateProfile($profile: ProfileInput!) {
    createProfile(profile: $profile) {
      id

      profile {
        username
        avatar
      }
    }
  }
`;

export const SEARCH_PROFILES = gql`
  query SearchProfiles($usernamePrefix: String!) {
    searchProfiles(usernamePrefix: $usernamePrefix) {
      id 

      profile {
        username
        avatar
      }
    }
  }
`;

export const GET_MY_PROFILE = gql`
  query GetMyProfile {
    me {
      id
      
      profile {
        username
        avatar
      }
    }
  }
`;
