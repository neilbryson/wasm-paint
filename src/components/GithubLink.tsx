import React, { FunctionComponent } from 'react';

const GITHUB_LINK = 'https://github.com/neilbryson/wasm-paint';

export const GithubLink: FunctionComponent = (props?: object) => (
  <a
    className="github-link"
    href={GITHUB_LINK}
    rel="nofollow noopener noreferrer"
    target="_blank"
    {...props}
  />
);
