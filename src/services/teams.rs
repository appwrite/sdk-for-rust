//! Teams service for Appwrite SDK

use crate::client::Client;

use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

/// The Teams service allows you to group users of your project and to enable
/// them to share read and write access to your project resources
#[derive(Debug, Clone)]
pub struct Teams {
    client: Client,
}

impl Teams {
    pub fn new(client: &Client) -> Self {
        Self { client: client.clone() }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    /// Get a list of all the teams in which the current user is a member. You can
    /// use the parameters to filter your results.
    pub async fn list(
        &self,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::TeamList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/teams".to_string();

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Create a new team. The user who creates the team will automatically be
    /// assigned as the owner of the team. Only the users with the owner role can
    /// invite new members, add new owners and delete or update the team.
    pub async fn create(
        &self,
        team_id: impl Into<String>,
        name: impl Into<String>,
        roles: Option<Vec<String>>,
    ) -> crate::error::Result<crate::models::Team> {
        let mut params = HashMap::new();
        params.insert("teamId".to_string(), json!(team_id.into()));
        params.insert("name".to_string(), json!(name.into()));
        if let Some(value) = roles {
            params.insert("roles".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/teams".to_string();

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a team by its ID. All team members have read access for this resource.
    pub async fn get(
        &self,
        team_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Team> {
        let params = HashMap::new();

        let path = "/teams/{teamId}".to_string().replace("{teamId}", &team_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update the team's name by its unique ID.
    pub async fn update_name(
        &self,
        team_id: impl Into<String>,
        name: impl Into<String>,
    ) -> crate::error::Result<crate::models::Team> {
        let mut params = HashMap::new();
        params.insert("name".to_string(), json!(name.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/teams/{teamId}".to_string().replace("{teamId}", &team_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

    /// Delete a team using its ID. Only team members with the owner role can
    /// delete the team.
    pub async fn delete(
        &self,
        team_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/teams/{teamId}".to_string().replace("{teamId}", &team_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to list a team's members using the team's ID. All team
    /// members have read access to this endpoint. Hide sensitive attributes from
    /// the response by toggling membership privacy in the Console.
    pub async fn list_memberships(
        &self,
        team_id: impl Into<String>,
        queries: Option<Vec<String>>,
        search: Option<&str>,
        total: Option<bool>,
    ) -> crate::error::Result<crate::models::MembershipList> {
        let mut params = HashMap::new();
        if let Some(value) = queries {
            params.insert("queries".to_string(), json!(value.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        }
        if let Some(value) = search {
            params.insert("search".to_string(), json!(value));
        }
        if let Some(value) = total {
            params.insert("total".to_string(), json!(value));
        }

        let path = "/teams/{teamId}/memberships".to_string().replace("{teamId}", &team_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Invite a new member to join your team. Provide an ID for existing users, or
    /// invite unregistered users using an email or phone number. If initiated from
    /// a Client SDK, Appwrite will send an email or sms with a link to join the
    /// team to the invited user, and an account will be created for them if one
    /// doesn't exist. If initiated from a Server SDK, the new member will be added
    /// automatically to the team.
    /// 
    /// You only need to provide one of a user ID, email, or phone number. Appwrite
    /// will prioritize accepting the user ID > email > phone number if you provide
    /// more than one of these parameters.
    /// 
    /// Use the `url` parameter to redirect the user from the invitation email to
    /// your app. After the user is redirected, use the [Update Team Membership
    /// Status](https://appwrite.io/docs/references/cloud/client-web/teams#updateMembershipStatus)
    /// endpoint to allow the user to accept the invitation to the team.
    /// 
    /// Please note that to avoid a [Redirect
    /// Attack](https://github.com/OWASP/CheatSheetSeries/blob/master/cheatsheets/Unvalidated_Redirects_and_Forwards_Cheat_Sheet.md)
    /// Appwrite will accept the only redirect URLs under the domains you have
    /// added as a platform on the Appwrite Console.
    #[allow(clippy::too_many_arguments)]
    pub async fn create_membership(
        &self,
        team_id: impl Into<String>,
        roles: impl IntoIterator<Item = impl Into<String>>,
        email: Option<&str>,
        user_id: Option<&str>,
        phone: Option<&str>,
        url: Option<&str>,
        name: Option<&str>,
    ) -> crate::error::Result<crate::models::Membership> {
        let mut params = HashMap::new();
        if let Some(value) = email {
            params.insert("email".to_string(), json!(value));
        }
        if let Some(value) = user_id {
            params.insert("userId".to_string(), json!(value));
        }
        if let Some(value) = phone {
            params.insert("phone".to_string(), json!(value));
        }
        params.insert("roles".to_string(), json!(roles.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        if let Some(value) = url {
            params.insert("url".to_string(), json!(value));
        }
        if let Some(value) = name {
            params.insert("name".to_string(), json!(value));
        }
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/teams/{teamId}/memberships".to_string().replace("{teamId}", &team_id.into().to_string());

        self.client.call(Method::POST, &path, Some(api_headers), Some(params)).await
    }

    /// Get a team member by the membership unique id. All team members have read
    /// access for this resource. Hide sensitive attributes from the response by
    /// toggling membership privacy in the Console.
    pub async fn get_membership(
        &self,
        team_id: impl Into<String>,
        membership_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Membership> {
        let params = HashMap::new();

        let path = "/teams/{teamId}/memberships/{membershipId}".to_string().replace("{teamId}", &team_id.into().to_string()).replace("{membershipId}", &membership_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Modify the roles of a team member. Only team members with the owner role
    /// have access to this endpoint. Learn more about [roles and
    /// permissions](https://appwrite.io/docs/permissions).
    pub async fn update_membership(
        &self,
        team_id: impl Into<String>,
        membership_id: impl Into<String>,
        roles: impl IntoIterator<Item = impl Into<String>>,
    ) -> crate::error::Result<crate::models::Membership> {
        let mut params = HashMap::new();
        params.insert("roles".to_string(), json!(roles.into_iter().map(|s| s.into()).collect::<Vec<String>>()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/teams/{teamId}/memberships/{membershipId}".to_string().replace("{teamId}", &team_id.into().to_string()).replace("{membershipId}", &membership_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// This endpoint allows a user to leave a team or for a team owner to delete
    /// the membership of any other team member. You can also use this endpoint to
    /// delete a user membership even if it is not accepted.
    pub async fn delete_membership(
        &self,
        team_id: impl Into<String>,
        membership_id: impl Into<String>,
    ) -> crate::error::Result<()> {
        let params = HashMap::new();
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/teams/{teamId}/memberships/{membershipId}".to_string().replace("{teamId}", &team_id.into().to_string()).replace("{membershipId}", &membership_id.into().to_string());

        self.client.call(Method::DELETE, &path, Some(api_headers), Some(params)).await
    }

    /// Use this endpoint to allow a user to accept an invitation to join a team
    /// after being redirected back to your app from the invitation email received
    /// by the user.
    /// 
    /// If the request is successful, a session for the user is automatically
    /// created.
    pub async fn update_membership_status(
        &self,
        team_id: impl Into<String>,
        membership_id: impl Into<String>,
        user_id: impl Into<String>,
        secret: impl Into<String>,
    ) -> crate::error::Result<crate::models::Membership> {
        let mut params = HashMap::new();
        params.insert("userId".to_string(), json!(user_id.into()));
        params.insert("secret".to_string(), json!(secret.into()));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/teams/{teamId}/memberships/{membershipId}/status".to_string().replace("{teamId}", &team_id.into().to_string()).replace("{membershipId}", &membership_id.into().to_string());

        self.client.call(Method::PATCH, &path, Some(api_headers), Some(params)).await
    }

    /// Get the team's shared preferences by its unique ID. If a preference doesn't
    /// need to be shared by all team members, prefer storing them in [user
    /// preferences](https://appwrite.io/docs/references/cloud/client-web/account#getPrefs).
    pub async fn get_prefs(
        &self,
        team_id: impl Into<String>,
    ) -> crate::error::Result<crate::models::Preferences> {
        let params = HashMap::new();

        let path = "/teams/{teamId}/prefs".to_string().replace("{teamId}", &team_id.into().to_string());

        self.client.call(Method::GET, &path, None, Some(params)).await
    }

    /// Update the team's preferences by its unique ID. The object you pass is
    /// stored as is and replaces any previous value. The maximum allowed prefs
    /// size is 64kB and throws an error if exceeded.
    pub async fn update_prefs(
        &self,
        team_id: impl Into<String>,
        prefs: serde_json::Value,
    ) -> crate::error::Result<crate::models::Preferences> {
        let mut params = HashMap::new();
        params.insert("prefs".to_string(), json!(prefs));
        let mut api_headers = HashMap::new();
        api_headers.insert("content-type".to_string(), "application/json".to_string());

        let path = "/teams/{teamId}/prefs".to_string().replace("{teamId}", &team_id.into().to_string());

        self.client.call(Method::PUT, &path, Some(api_headers), Some(params)).await
    }

}

impl crate::services::Service for Teams {
    fn client(&self) -> &Client {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teams_creation() {
        let client = Client::new();
        let service = Teams::new(&client);
        assert!(service.client().endpoint().contains("cloud.appwrite.io/v1"));
    }
}
