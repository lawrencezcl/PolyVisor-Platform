import React from 'react'
import React from 'react'
import { Box, Typography, Card, CardContent, Grid, LinearProgress, Chip } from '@mui/material'

const NetworkHealth: React.FC = () => {
  // 模拟数据
  const healthData = {
    overall_score: 92,
    network_status: 'healthy',
    metrics: {
      connectivity_score: 95,
      throughput_score: 88,
      latency_score: 90,
      consensus_score: 94,
      availability_score: 96
    },
    warnings: [
      {
        level: 'warning',
        message: '部分节点响应时间较慢',
        component: 'network',
        timestamp: new Date().toISOString()
      }
    ]
  }

  const getScoreColor = (score: number) => {
    if (score >= 90) return 'success'
    if (score >= 70) return 'warning'
    return 'error'
  }

  return (
    <Box>
      <Typography variant="h4" component="h1" gutterBottom>
        网络健康状况
      </Typography>
      <Typography variant="body1" color="text.secondary" paragraph>
        监控网络各项指标的健康状况
      </Typography>

      <Grid container spacing={3}>
        {/* 总体健康度 */}
        <Grid item xs={12} md={4}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                总体健康度
              </Typography>
              <Box display="flex" alignItems="center" gap={2} mb={2}>
                <Typography variant="h3" color="primary">
                  {healthData.overall_score}%
                </Typography>
                <Chip 
                  label="健康" 
                  color="success" 
                  variant="outlined"
                />
              </Box>
              <LinearProgress 
                variant="determinate" 
                value={healthData.overall_score}
                sx={{ height: 8, borderRadius: 4 }}
              />
            </CardContent>
          </Card>
        </Grid>

        {/* 详细指标 */}
        <Grid item xs={12} md={8}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                详细指标
              </Typography>
              <Grid container spacing={2}>
                <Grid item xs={12} sm={6}>
                  <Box mb={2}>
                    <Box display="flex" justifyContent="space-between" mb={1}>
                      <Typography variant="body2">连通性</Typography>
                      <Typography variant="body2" color="primary">
                        {healthData.metrics.connectivity_score}%
                      </Typography>
                    </Box>
                    <LinearProgress 
                      variant="determinate" 
                      value={healthData.metrics.connectivity_score}
                      color={getScoreColor(healthData.metrics.connectivity_score) as any}
                    />
                  </Box>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Box mb={2}>
                    <Box display="flex" justifyContent="space-between" mb={1}>
                      <Typography variant="body2">吞吐量</Typography>
                      <Typography variant="body2" color="primary">
                        {healthData.metrics.throughput_score}%
                      </Typography>
                    </Box>
                    <LinearProgress 
                      variant="determinate" 
                      value={healthData.metrics.throughput_score}
                      color={getScoreColor(healthData.metrics.throughput_score) as any}
                    />
                  </Box>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Box mb={2}>
                    <Box display="flex" justifyContent="space-between" mb={1}>
                      <Typography variant="body2">延迟</Typography>
                      <Typography variant="body2" color="primary">
                        {healthData.metrics.latency_score}%
                      </Typography>
                    </Box>
                    <LinearProgress 
                      variant="determinate" 
                      value={healthData.metrics.latency_score}
                      color={getScoreColor(healthData.metrics.latency_score) as any}
                    />
                  </Box>
                </Grid>
                <Grid item xs={12} sm={6}>
                  <Box mb={2}>
                    <Box display="flex" justifyContent="space-between" mb={1}>
                      <Typography variant="body2">共识</Typography>
                      <Typography variant="body2" color="primary">
                        {healthData.metrics.consensus_score}%
                      </Typography>
                    </Box>
                    <LinearProgress 
                      variant="determinate" 
                      value={healthData.metrics.consensus_score}
                      color={getScoreColor(healthData.metrics.consensus_score) as any}
                    />
                  </Box>
                </Grid>
              </Grid>
            </CardContent>
          </Card>
        </Grid>

        {/* 警告信息 */}
        <Grid item xs={12}>
          <Card>
            <CardContent>
              <Typography variant="h6" gutterBottom>
                状态警告
              </Typography>
              {healthData.warnings.map((warning, index) => (
                <Box key={index} display="flex" alignItems="center" gap={2} py={1}>
                  <Chip 
                    label={warning.level === 'warning' ? '警告' : '信息'}
                    color={warning.level === 'warning' ? 'warning' : 'info'}
                    size="small"
                  />
                  <Typography variant="body2" flex={1}>
                    {warning.message}
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    {new Date(warning.timestamp).toLocaleTimeString()}
                  </Typography>
                </Box>
              ))}
            </CardContent>
          </Card>
        </Grid>
      </Grid>
    </Box>
  )
}

export default NetworkHealth