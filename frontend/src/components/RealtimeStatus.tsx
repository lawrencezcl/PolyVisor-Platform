import React from 'react'
import { Box, Chip, Typography } from '@mui/material'
import { WifiOutlined, WifiOffOutlined } from '@mui/icons-material'

interface RealtimeStatusProps {
  isConnected: boolean
}

const RealtimeStatus: React.FC<RealtimeStatusProps> = ({ isConnected }) => {
  return (
    <Box display="flex" alignItems="center" gap={1}>
      <Chip
        icon={isConnected ? <WifiOutlined /> : <WifiOffOutlined />}
        label={
          <Box display="flex" alignItems="center" gap={0.5}>
            <Box
              sx={{
                width: 8,
                height: 8,
                borderRadius: '50%',
                backgroundColor: isConnected ? 'success.main' : 'error.main',
                animation: isConnected ? 'pulse 2s ease-in-out infinite' : 'none',
              }}
            />
            <Typography variant="caption">
              {isConnected ? '实时连接' : '连接断开'}
            </Typography>
          </Box>
        }
        variant="outlined"
        size="small"
        color={isConnected ? 'success' : 'error'}
      />
    </Box>
  )
}

export default RealtimeStatus