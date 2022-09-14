import React, { FunctionComponent } from 'react';
import GitHubLogo from '../assets/images/GitHub_Logo.png';

const GITHUB_LINK = 'https://github.com/neilbryson/wasm-paint';

export const GithubLink: FunctionComponent = () => (
  <a
    className="github-link"
    href={GITHUB_LINK}
    rel="nofollow noopener noreferrer"
    target="_blank"
  >
    <img src={GitHubLogo} alt="GitHub Logo" />
  </a>
);
