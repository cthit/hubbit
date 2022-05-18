import React from 'react';

import styles from './UserStatsCard.module.scss';

interface UserStatCardProps {
  title: string;
  content: string;
}

const UserStatsCard = ({ title, content }: UserStatCardProps) => (
  <div className={styles.infoContainer}>
    <div className={styles.infoHeader}>{title}</div>
    <div className={styles.infoText}>{content}</div>
  </div>
);

export default UserStatsCard;
