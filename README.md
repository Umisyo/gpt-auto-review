**GPT AUTO REVIEW**

This workflow automatically reviews pull requests when they are opened, synced, or labeled.

## Usage

1. Place this workflow in the `.github/workflows` directory of your repository.
2. Add `GPT_API_KEY` to your GitHub repository secrets.
3. Create or update a pull request and label it `auto-review`.
4. The workflow will run and review automatically. Review results are displayed as pull request comments.

## Workflow Details

My Workflow consists of the following jobs.

1. **gpt-auto-review**: Runs when a pull request is open, synced, or labeled `auto-review`. This job performs the following steps:
   - Checkout repository
   - Get pull request information
   - Install Rust
   - run build
   - Run `gpt-auto-review` to review the pull request.
   - Post the review result as a pull request comment.

Now let's use My Workflow to easily perform automated pull request reviews.
