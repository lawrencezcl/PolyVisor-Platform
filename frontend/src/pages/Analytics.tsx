import React, { useState, useEffect } from 'react'
import {
  Box,
  Typography,
  Paper,
  Card,
  CardContent,
  Select,
  MenuItem,
  FormControl,
  InputLabel,
  Button,
  Chip,
  LinearProgress,
  Alert
} from '@mui/material'
import {
  TrendingUp,
  Security,
  Visibility,
  BarChart,
  ShowChart
} from '@mui/icons-material'
import {
  LineChart,
  Line,
  AreaChart,
  Area,
  PieChart as RechartsPieChart,
  Pie,
  Cell,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
  Legend
} from 'recharts'
import { useApi } from '../contexts/ApiContext'
import { useWebSocket } from '../contexts/WebSocketContext'

interface AnalyticsData {
  networkTrends: Array<{
    timestamp: string
    transactions: number
    blocks: number
    validators: number
    privacy_score: number
  }>
  privacyMetrics: {
    anonymity_set_size: number
    mixing_effectiveness: number
    privacy_level: 'high' | 'medium' | 'low'
    zk_proofs_generated: number
  }
  zkProofStats: {
    total_proofs: number
    verification_rate: number
    proof_types: Array<{ name: string; count: number; color: string }>
  }
  predictions: {
    network_growth: number
    privacy_adoption: number
    transaction_volume: number
    confidence_score: number
  }
}

const COLORS = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042', '#8884D8']

const Analytics: React.FC = () => {
  const [analyticsData, setAnalyticsData] = useState<AnalyticsData | null>(null)
  const [loading, setLoading] = useState(true)
  const [timeRange, setTimeRange] = useState('7d')
  const [analysisType, setAnalysisType] = useState('network')
  const { api } = useApi()
  const { lastMessage } = useWebSocket()

  useEffect(() => {
    fetchAnalyticsData()
  }, [timeRange, analysisType])

  const fetchAnalyticsData = async () => {
    try {
      setLoading(true)
      // 模拟数据
      setAnalyticsData({
        networkTrends: [
          { timestamp: '2023-12-01', transactions: 1250, blocks: 45, validators: 120, privacy_score: 85 },
          { timestamp: '2023-12-02', transactions: 1380, blocks: 52, validators: 125, privacy_score: 88 },
          { timestamp: '2023-12-03', transactions: 1420, blocks: 48, validators: 128, privacy_score: 90 }
        ],
        privacyMetrics: {
          anonymity_set_size: 15000,
          mixing_effectiveness: 0.92,
          privacy_level: 'high',
          zk_proofs_generated: 8500
        },
        zkProofStats: {
          total_proofs: 25000,
          verification_rate: 0.98,
          proof_types: [
            { name: 'Transaction', count: 12000, color: '#0088FE' },
            { name: 'Identity', count: 8000, color: '#00C49F' },
            { name: 'Membership', count: 5000, color: '#FFBB28' }
          ]
        },
        predictions: {
          network_growth: 0.15,
          privacy_adoption: 0.25,
          transaction_volume: 0.18,
          confidence_score: 0.82
        }
      })
    } catch (error) {
      console.error('获取分析数据失败:', error)
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return (
      <Box sx={{ p: 3 }}>
        <Typography variant="h4" gutterBottom>数据分析</Typography>
        <LinearProgress />
        <Typography sx={{ mt: 2 }}>加载分析数据中...</Typography>
      </Box>
    )
  }

  if (!analyticsData) {
    return (
      <Box sx={{ p: 3 }}>
        <Alert severity="error">无法加载分析数据</Alert>
      </Box>
    )
  }

  return (
    <Box sx={{ p: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4" component="h1">
          <BarChart sx={{ mr: 1, verticalAlign: 'middle' }} />
          数据分析
        </Typography>
        <Box sx={{ display: 'flex', gap: 2 }}>
          <FormControl size="small" sx={{ minWidth: 120 }}>
            <InputLabel>时间范围</InputLabel>
            <Select value={timeRange} label="时间范围" onChange={(e) => setTimeRange(e.target.value)}>
              <MenuItem value="1d">1天</MenuItem>
              <MenuItem value="7d">7天</MenuItem>
              <MenuItem value="30d">30天</MenuItem>
              <MenuItem value="90d">90天</MenuItem>
            </Select>
          </FormControl>
          <FormControl size="small" sx={{ minWidth: 120 }}>
            <InputLabel>分析类型</InputLabel>
            <Select value={analysisType} label="分析类型" onChange={(e) => setAnalysisType(e.target.value)}>
              <MenuItem value="network">网络分析</MenuItem>
              <MenuItem value="privacy">隐私分析</MenuItem>
              <MenuItem value="performance">性能分析</MenuItem>
              <MenuItem value="security">安全分析</MenuItem>
            </Select>
          </FormControl>
          <Button variant="outlined">导出数据</Button>
        </Box>
      </Box>

      {/* 网络趋势分析 */}
      <Paper sx={{ p: 3, mb: 3 }}>
        <Typography variant="h6" gutterBottom>
          <TrendingUp sx={{ mr: 1, verticalAlign: 'middle' }} />
          网络趋势分析
        </Typography>
        <Box sx={{ height: 400, mt: 2 }}>
          <ResponsiveContainer width="100%" height="100%">
            <LineChart data={analyticsData.networkTrends}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="timestamp" />
              <YAxis />
              <Tooltip />
              <Legend />
              <Line type="monotone" dataKey="transactions" stroke="#8884d8" strokeWidth={2} name="交易数量" />
              <Line type="monotone" dataKey="blocks" stroke="#82ca9d" strokeWidth={2} name="区块数量" />
              <Line type="monotone" dataKey="validators" stroke="#ffc658" strokeWidth={2} name="验证者数量" />
              <Line type="monotone" dataKey="privacy_score" stroke="#ff7300" strokeWidth={2} name="隐私评分" />
            </LineChart>
          </ResponsiveContainer>
        </Box>
      </Paper>

      {/* 核心指标 */}
      <Box sx={{ display: 'flex', gap: 3, mb: 3, flexWrap: 'wrap' }}>
        <Paper sx={{ p: 3, flex: 1, minWidth: 300 }}>
          <Typography variant="h6" gutterBottom>
            <Security sx={{ mr: 1, verticalAlign: 'middle' }} />
            隐私分析
          </Typography>
          <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 2 }}>
            <Card>
              <CardContent>
                <Typography color="textSecondary" gutterBottom>匿名集大小</Typography>
                <Typography variant="h5">{analyticsData.privacyMetrics.anonymity_set_size.toLocaleString()}</Typography>
              </CardContent>
            </Card>
            <Card>
              <CardContent>
                <Typography color="textSecondary" gutterBottom>混合有效性</Typography>
                <Typography variant="h5">{(analyticsData.privacyMetrics.mixing_effectiveness * 100).toFixed(1)}%</Typography>
              </CardContent>
            </Card>
            <Card>
              <CardContent>
                <Typography color="textSecondary" gutterBottom>隐私等级</Typography>
                <Chip
                  label={analyticsData.privacyMetrics.privacy_level}
                  color={analyticsData.privacyMetrics.privacy_level === 'high' ? 'success' : 'warning'}
                />
              </CardContent>
            </Card>
            <Card>
              <CardContent>
                <Typography color="textSecondary" gutterBottom>ZK证明数量</Typography>
                <Typography variant="h5">{analyticsData.privacyMetrics.zk_proofs_generated.toLocaleString()}</Typography>
              </CardContent>
            </Card>
          </Box>
        </Paper>

        <Paper sx={{ p: 3, flex: 1, minWidth: 300 }}>
          <Typography variant="h6" gutterBottom>
            <Visibility sx={{ mr: 1, verticalAlign: 'middle' }} />
            零知识证明统计
          </Typography>
          <Box sx={{ height: 250, mt: 2 }}>
            <ResponsiveContainer width="100%" height="100%">
              <RechartsPieChart>
                <Pie
                  data={analyticsData.zkProofStats.proof_types}
                  cx="50%"
                  cy="50%"
                  outerRadius={80}
                  dataKey="count"
                >
                  {analyticsData.zkProofStats.proof_types.map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={entry.color || COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
                <Tooltip />
                <Legend />
              </RechartsPieChart>
            </ResponsiveContainer>
          </Box>
          <Box sx={{ display: 'flex', justifyContent: 'space-between', mt: 2 }}>
            <Box>
              <Typography color="textSecondary" gutterBottom>总证明数量</Typography>
              <Typography variant="h6">{analyticsData.zkProofStats.total_proofs.toLocaleString()}</Typography>
            </Box>
            <Box>
              <Typography color="textSecondary" gutterBottom>验证成功率</Typography>
              <Typography variant="h6">{(analyticsData.zkProofStats.verification_rate * 100).toFixed(2)}%</Typography>
            </Box>
          </Box>
        </Paper>
      </Box>

      {/* 预测模型 */}
      <Paper sx={{ p: 3 }}>
        <Typography variant="h6" gutterBottom>
          <ShowChart sx={{ mr: 1, verticalAlign: 'middle' }} />
          预测分析
        </Typography>
        <Box sx={{ display: 'flex', gap: 3, flexWrap: 'wrap' }}>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>网络增长预测</Typography>
              <Typography variant="h4" color="primary">+{(analyticsData.predictions.network_growth * 100).toFixed(1)}%</Typography>
              <LinearProgress variant="determinate" value={analyticsData.predictions.network_growth * 100} sx={{ mt: 1 }} />
            </CardContent>
          </Card>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>隐私采用率预测</Typography>
              <Typography variant="h4" color="secondary">+{(analyticsData.predictions.privacy_adoption * 100).toFixed(1)}%</Typography>
              <LinearProgress variant="determinate" value={analyticsData.predictions.privacy_adoption * 100} color="secondary" sx={{ mt: 1 }} />
            </CardContent>
          </Card>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>交易量预测</Typography>
              <Typography variant="h4" color="info">+{(analyticsData.predictions.transaction_volume * 100).toFixed(1)}%</Typography>
              <LinearProgress variant="determinate" value={analyticsData.predictions.transaction_volume * 100} color="info" sx={{ mt: 1 }} />
            </CardContent>
          </Card>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>预测置信度</Typography>
              <Typography variant="h4" color="success">{(analyticsData.predictions.confidence_score * 100).toFixed(1)}%</Typography>
              <LinearProgress variant="determinate" value={analyticsData.predictions.confidence_score * 100} color="success" sx={{ mt: 1 }} />
            </CardContent>
          </Card>
        </Box>
      </Paper>
    </Box>
  )
}

export default Analytics