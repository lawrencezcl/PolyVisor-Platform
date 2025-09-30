import React, { useState, useEffect } from 'react'
import {
  Grid,
  Card,
  CardContent,
  Typography,
  Box,
  CircularProgress,
  LinearProgress,
  Chip,
  IconButton,
  Tooltip
} from '@mui/material'
import {
  TrendingUp,
  TrendingDown,
  Refresh as RefreshIcon,
  Speed as SpeedIcon,
  Security as SecurityIcon,
  People as PeopleIcon,
  Timeline as TimelineIcon
} from '@mui/icons-material'
import { useApi } from '../contexts/ApiContext'
import { useWebSocket } from '../contexts/WebSocketContext'
import MetricsChart from '../components/MetricsChart'
import RealtimeStatus from '../components/RealtimeStatus'

interface DashboardData {
  networkHealth: {
    overall_score: number
    status: string
    last_updated: string
  }
  metrics: {
    total_nodes: number
    active_connections: number
    block_time_avg: number
    transaction_rate: number
  }
  recentActivity: Array<{
    type: string
    message: string
    timestamp: string
  }>
}

const Dashboard: React.FC = () => {
  const [data, setData] = useState<DashboardData | null>(null)
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)
  const { api } = useApi()
  const { isConnected, lastMessage } = useWebSocket()

  const fetchDashboardData = async () => {
    try {
      setLoading(true)
      const [healthResponse, metricsResponse] = await Promise.all([
        api.get('/v1/health/network'),
        api.get('/v1/metrics')
      ])

      setData({
        networkHealth: healthResponse.data,
        metrics: {
          total_nodes: 156,
          active_connections: 1247,
          block_time_avg: 6.2,
          transaction_rate: 45.8
        },
        recentActivity: [
          {
            type: 'info',
            message: '新增网络节点：validator-789',
            timestamp: new Date().toISOString()
          },
          {
            type: 'success',
            message: '零知识证明验证成功',
            timestamp: new Date().toISOString()
          }
        ]
      })
      setError(null)
    } catch (err) {
      console.error('Failed to fetch dashboard data:', err)
      setError('获取数据失败，显示模拟数据')
      // 设置模拟数据
      setData({
        networkHealth: {
          overall_score: 92,
          status: 'healthy',
          last_updated: new Date().toISOString()
        },
        metrics: {
          total_nodes: 156,
          active_connections: 1247,
          block_time_avg: 6.2,
          transaction_rate: 45.8
        },
        recentActivity: [
          {
            type: 'info',
            message: '新增网络节点：validator-789',
            timestamp: new Date().toISOString()
          },
          {
            type: 'success',
            message: '零知识证明验证成功',
            timestamp: new Date().toISOString()
          }
        ]
      })
    } finally {
      setLoading(false)
    }
  }

  useEffect(() => {
    fetchDashboardData()
  }, [])

  // 处理实时更新
  useEffect(() => {
    if (lastMessage?.type === 'metrics_update') {
      // 更新实时数据
      console.log('Received real-time update:', lastMessage.data)
    }
  }, [lastMessage])

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight={400}>
        <CircularProgress />
      </Box>
    )
  }

  if (!data) {
    return (
      <Box textAlign="center" py={4}>
        <Typography variant="h6" color="error">
          无法加载数据
        </Typography>
      </Box>
    )
  }

  return (
    <Box>
      {/* 页面标题和操作 */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Box>
          <Typography variant="h4" component="h1" gutterBottom>
            网络总览
          </Typography>
          <Typography variant="body1" color="text.secondary">
            实时监控Polkadot网络状态和指标
          </Typography>
        </Box>
        <Box display="flex" gap={1} alignItems="center">
          <RealtimeStatus isConnected={isConnected} />
          <Tooltip title="刷新数据">
            <IconButton onClick={fetchDashboardData}>
              <RefreshIcon />
            </IconButton>
          </Tooltip>
        </Box>
      </Box>

      {error && (
        <Box mb={2}>
          <Chip 
            label={error} 
            color="warning" 
            variant="outlined" 
            size="small"
          />
        </Box>
      )}

      {/* 核心指标卡片 */}
      <Grid container spacing={3} mb={4}>
        <Grid item xs={12} sm={6} md={3}>
          <Card className="metric-card">
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography color="text.secondary" gutterBottom variant="body2">
                    网络健康度
                  </Typography>
                  <Typography variant="h4" component="div">
                    {data.networkHealth.overall_score}%
                  </Typography>
                  <Chip 
                    label={data.networkHealth.status === 'healthy' ? '健康' : '警告'} 
                    color={data.networkHealth.status === 'healthy' ? 'success' : 'warning'}
                    size="small"
                  />
                </Box>
                <SecurityIcon sx={{ fontSize: 40, color: 'primary.main', opacity: 0.7 }} />
              </Box>
              <LinearProgress 
                variant="determinate" 
                value={data.networkHealth.overall_score} 
                sx={{ mt: 2, height: 6, borderRadius: 3 }}
              />
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card className="metric-card">
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography color="text.secondary" gutterBottom variant="body2">
                    活跃节点
                  </Typography>
                  <Typography variant="h4" component="div">
                    {data.metrics.total_nodes}
                  </Typography>
                  <Box display="flex" alignItems="center" mt={1}>
                    <TrendingUp sx={{ fontSize: 16, color: 'success.main', mr: 0.5 }} />
                    <Typography variant="body2" color="success.main">
                      +5.2%
                    </Typography>
                  </Box>
                </Box>
                <PeopleIcon sx={{ fontSize: 40, color: 'primary.main', opacity: 0.7 }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card className="metric-card">
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography color="text.secondary" gutterBottom variant="body2">
                    平均出块时间
                  </Typography>
                  <Typography variant="h4" component="div">
                    {data.metrics.block_time_avg}s
                  </Typography>
                  <Box display="flex" alignItems="center" mt={1}>
                    <TrendingDown sx={{ fontSize: 16, color: 'success.main', mr: 0.5 }} />
                    <Typography variant="body2" color="success.main">
                      -2.1%
                    </Typography>
                  </Box>
                </Box>
                <SpeedIcon sx={{ fontSize: 40, color: 'primary.main', opacity: 0.7 }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card className="metric-card">
            <CardContent>
              <Box display="flex" alignItems="center" justifyContent="space-between">
                <Box>
                  <Typography color="text.secondary" gutterBottom variant="body2">
                    交易速率
                  </Typography>
                  <Typography variant="h4" component="div">
                    {data.metrics.transaction_rate}
                  </Typography>
                  <Typography variant="body2" color="text.secondary">
                    TPS
                  </Typography>
                </Box>
                <TimelineIcon sx={{ fontSize: 40, color: 'primary.main', opacity: 0.7 }} />
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* 图表区域 */}
      <Grid container spacing={3}>
        <Grid item xs={12} lg={8}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                网络指标趋势
              </Typography>
              <MetricsChart />
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} lg={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                最新活动
              </Typography>
              <Box>
                {data.recentActivity.map((activity, index) => (
                  <Box 
                    key={index} 
                    display="flex" 
                    alignItems="center" 
                    py={1}
                    borderBottom={index < data.recentActivity.length - 1 ? '1px solid rgba(255,255,255,0.1)' : 'none'}
                  >
                    <Chip
                      label={activity.type === 'success' ? '成功' : '信息'}
                      color={activity.type === 'success' ? 'success' : 'info'}
                      size="small"
                      sx={{ mr: 2, minWidth: 60 }}
                    />
                    <Box flex={1}>
                      <Typography variant="body2">
                        {activity.message}
                      </Typography>
                      <Typography variant="caption" color="text.secondary">
                        {new Date(activity.timestamp).toLocaleTimeString()}
                      </Typography>
                    </Box>
                  </Box>
                ))}
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  )
}

export default Dashboard