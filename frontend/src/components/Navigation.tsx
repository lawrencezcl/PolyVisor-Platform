import React from 'react'
import { 
  Drawer, 
  List, 
  ListItem, 
  ListItemButton, 
  ListItemIcon, 
  ListItemText,
  Typography,
  Box,
  Divider,
  Chip
} from '@mui/material'
import {
  Dashboard as DashboardIcon,
  Health as HealthIcon,
  Security as SecurityIcon,
  Analytics as AnalyticsIcon,
  People as PeopleIcon,
  CircleOutlined as StatusIcon
} from '@mui/icons-material'
import { useNavigate, useLocation } from 'react-router-dom'
import { useWebSocket } from '../contexts/WebSocketContext'

const drawerWidth = 280

const menuItems = [
  { text: '总览', icon: <DashboardIcon />, path: '/' },
  { text: '网络健康', icon: <HealthIcon />, path: '/health' },
  { text: '隐私设置', icon: <SecurityIcon />, path: '/privacy' },
  { text: '数据分析', icon: <AnalyticsIcon />, path: '/analytics' },
  { text: '贡献者', icon: <PeopleIcon />, path: '/contributors' },
]

const Navigation: React.FC = () => {
  const navigate = useNavigate()
  const location = useLocation()
  const { isConnected } = useWebSocket()

  return (
    <Drawer
      variant="permanent"
      sx={{
        width: drawerWidth,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: drawerWidth,
          boxSizing: 'border-box',
          backgroundColor: 'background.paper',
          borderRight: '1px solid rgba(255, 255, 255, 0.12)',
        },
      }}
    >
      <Box sx={{ p: 3, textAlign: 'center' }}>
        <Typography 
          variant="h4" 
          component="div" 
          className="gradient-text"
          sx={{ fontWeight: 'bold', mb: 1 }}
        >
          PolyVisor
        </Typography>
        <Typography 
          variant="body2" 
          color="text.secondary"
          sx={{ mb: 2 }}
        >
          隐私保护的Polkadot网络分析平台
        </Typography>
        <Chip
          icon={<StatusIcon />}
          label={isConnected ? '实时连接' : '连接断开'}
          color={isConnected ? 'success' : 'error'}
          variant="outlined"
          size="small"
          className={isConnected ? 'pulse-animation' : ''}
        />
      </Box>
      
      <Divider />
      
      <List sx={{ px: 2, mt: 2 }}>
        {menuItems.map((item) => {
          const isActive = location.pathname === item.path
          return (
            <ListItem key={item.text} disablePadding sx={{ mb: 1 }}>
              <ListItemButton
                onClick={() => navigate(item.path)}
                selected={isActive}
                sx={{
                  borderRadius: 2,
                  '&.Mui-selected': {
                    backgroundColor: 'rgba(0, 212, 255, 0.1)',
                    '&:hover': {
                      backgroundColor: 'rgba(0, 212, 255, 0.15)',
                    },
                  },
                  '&:hover': {
                    backgroundColor: 'rgba(255, 255, 255, 0.05)',
                  },
                }}
              >
                <ListItemIcon 
                  sx={{ 
                    color: isActive ? 'primary.main' : 'text.secondary',
                    minWidth: 40 
                  }}
                >
                  {item.icon}
                </ListItemIcon>
                <ListItemText 
                  primary={item.text}
                  sx={{
                    '& .MuiListItemText-primary': {
                      color: isActive ? 'primary.main' : 'text.primary',
                      fontWeight: isActive ? 600 : 400,
                    }
                  }}
                />
              </ListItemButton>
            </ListItem>
          )
        })}
      </List>

      <Box sx={{ mt: 'auto', p: 2 }}>
        <Box 
          className="glass-effect"
          sx={{ 
            p: 2, 
            borderRadius: 2,
            textAlign: 'center'
          }}
        >
          <Typography variant="caption" color="text.secondary">
            版本 v0.1.0
          </Typography>
          <br />
          <Typography variant="caption" color="text.secondary">
            © 2024 PolyVisor Team
          </Typography>
        </Box>
      </Box>
    </Drawer>
  )
}

export default Navigation