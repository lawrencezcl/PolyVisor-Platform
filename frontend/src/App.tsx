import React from 'react'
import { Routes, Route, Link, useLocation } from 'react-router-dom'
import { Box, Container, Typography, Card, CardContent, Drawer, List, ListItem, ListItemIcon, ListItemText, AppBar, Toolbar } from '@mui/material'
import { Dashboard as DashboardIcon, HealthAndSafety, Security, Analytics as AnalyticsIcon, People } from '@mui/icons-material'
// import { ApiProvider } from './contexts/ApiContext'
// import { WebSocketProvider } from './contexts/WebSocketContext'

// 临时简化的API和WebSocket providers
const ApiProvider = ({ children }: { children: React.ReactNode }) => <>{children}</>
const WebSocketProvider = ({ children }: { children: React.ReactNode }) => <>{children}</>

// 导航组件
const Navigation = () => {
  const location = useLocation()
  
  const menuItems = [
    { path: '/', label: '仪表板', icon: <DashboardIcon /> },
    { path: '/health', label: '网络健康', icon: <HealthAndSafety /> },
    { path: '/privacy', label: '隐私设置', icon: <Security /> },
    { path: '/analytics', label: '数据分析', icon: <AnalyticsIcon /> },
    { path: '/contributors', label: '贡献者', icon: <People /> },
  ]

  return (
    <Drawer
      variant="permanent"
      sx={{
        width: 240,
        flexShrink: 0,
        '& .MuiDrawer-paper': {
          width: 240,
          boxSizing: 'border-box',
        },
      }}
    >
      <Toolbar>
        <Typography variant="h6" noWrap component="div">
          PolyVisor
        </Typography>
      </Toolbar>
      <Box sx={{ overflow: 'auto' }}>
        <List>
          {menuItems.map((item) => (
            <ListItem
              key={item.path}
              component={Link}
              to={item.path}
              sx={{
                backgroundColor: location.pathname === item.path ? 'primary.main' : 'transparent',
                '&:hover': {
                  backgroundColor: 'primary.dark',
                },
              }}
            >
              <ListItemIcon sx={{ color: 'inherit' }}>
                {item.icon}
              </ListItemIcon>
              <ListItemText primary={item.label} />
            </ListItem>
          ))}
        </List>
      </Box>
    </Drawer>
  )
}

// 临时简化的页面组件
const Dashboard = () => (
  <Card>
    <CardContent>
      <Typography variant="h4" gutterBottom>
        PolyVisor Dashboard
      </Typography>
      <Typography variant="body1">
        欢迎使用PolyVisor - 隐私保护的Polkadot网络分析平台
      </Typography>
    </CardContent>
  </Card>
)

const NetworkHealth = () => (
  <Card>
    <CardContent>
      <Typography variant="h4" gutterBottom>
        网络健康状况
      </Typography>
      <Typography variant="body1">
        网络状态监控页面
      </Typography>
    </CardContent>
  </Card>
)

const PrivacySettings = () => (
  <Card>
    <CardContent>
      <Typography variant="h4" gutterBottom>
        隐私设置
      </Typography>
      <Typography variant="body1">
        隐私配置页面
      </Typography>
    </CardContent>
  </Card>
)

const Analytics = () => (
  <Card>
    <CardContent>
      <Typography variant="h4" gutterBottom>
        数据分析
      </Typography>
      <Typography variant="body1">
        分析统计页面
      </Typography>
    </CardContent>
  </Card>
)

const Contributors = () => (
  <Card>
    <CardContent>
      <Typography variant="h4" gutterBottom>
        贡献者
      </Typography>
      <Typography variant="body1">
        贡献者管理页面
      </Typography>
    </CardContent>
  </Card>
)

function App() {
  return (
    <ApiProvider>
      <WebSocketProvider>
        <Box sx={{ display: 'flex', minHeight: '100vh' }}>
          <Navigation />
          <Box
            component="main"
            sx={{
              flexGrow: 1,
              backgroundColor: 'background.default',
              overflow: 'auto',
              p: 3,
              ml: '240px' // 留出导航栏空间
            }}
          >
            <Container maxWidth="xl">
              <Routes>
                <Route path="/" element={<Dashboard />} />
                <Route path="/health" element={<NetworkHealth />} />
                <Route path="/privacy" element={<PrivacySettings />} />
                <Route path="/analytics" element={<Analytics />} />
                <Route path="/contributors" element={<Contributors />} />
              </Routes>
            </Container>
          </Box>
        </Box>
      </WebSocketProvider>
    </ApiProvider>
  )
}

export default App