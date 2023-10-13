variable "github_token" {
  type = string
  description = "Your Github OAuth token"
}

variable "github_owner" {
  type = string
  description = "The owner of the Github repository"
}

variable "github_name" {
  type = string
  description = "The name of the Github repository"
}

variable "github_visibility" {
  type = string
  default = "private"
  description = "The visibility of the Github repository (public or private)"
}

variable "gha_op_service_account" {
  type = string
  description = "A 1Password service account token with access to the right vault"
}

terraform {
  required_providers {
    github = {
      source  = "integrations/github"
      version = "~> 5.0"
    }
  }
}

provider "github" {
  token = var.github_token
  owner = var.github_owner
}

resource "github_repository" "repository" {
  name = var.github_name
  visibility = var.github_visibility

  has_issues = true
  has_projects = false
  has_wiki = false
  has_downloads = false
  vulnerability_alerts = true
}

resource "github_actions_secret" "op_service_account" {
  repository = var.github_name

  secret_name = "OP_SERVICE_ACCOUNT"
  plaintext_value = var.gha_op_service_account
}
