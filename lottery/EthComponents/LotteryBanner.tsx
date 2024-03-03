import React from 'react';
import './LotteryBanner.scss';

interface LotteryBannerProps {
  words: string
}
function LotteryBanner(props: LotteryBannerProps) {
  return (
    <div className="lottery-banner">
      <div className="content">
        <span className="highlight">{props.words}</span>
      </div>
    </div>
  );
};

export default LotteryBanner;
