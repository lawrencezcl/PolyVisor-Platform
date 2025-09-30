import React from 'react'
import React from 'react'
import { 
  Box, 
  Typography, 
  Card, 
  CardContent, 
  Grid,
  Tabs,
  Tab,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Chip,
  Avatar,
  Paper
} from '@mui/material'
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, PieChart, Pie, Cell } from 'recharts'

interface TabPanelProps {
  children?: React.ReactNode
  index: number
  value: number
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props
  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`analytics-tabpanel-${index}`}
      aria-labelledby={`analytics-tab-${index}`}
      {...other}
    >
      {value === index && <Box sx={{ p: 3 }}>{children}</Box>}
    </div>
  )
}

const Contributors: React.FC = () => {
  const [tabValue, setTabValue] = React.useState(0)

  // 模拟数据
  const contributorsData = [
    {
      address: '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKv3gB',
      displayName: 'Validator Node 001',
      type: 'validator',
      contributions: 1247,
      quality: 95,
      reputation: 892,
      lastActive: '2024-01-15T10:30:00Z'
    },
    {
      address: '5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty',
      displayName: 'Data Provider Alpha',
      type: 'data_provider',
      contributions: 856,
      quality: 88,
      reputation: 654,
      lastActive: '2024-01-15T09:15:00Z'
    },
    {
      address: '5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy',
      displayName: 'Research Institute',
      type: 'researcher',
      contributions: 432,
      quality: 98,
      reputation: 1123,
      lastActive: '2024-01-15T08:45:00Z'
    }
  ]

  const contributionStats = [
    { name: '验证节点', value: 45, color: '#00d4ff' },
    { name: '数据提供商', value: 32, color: '#ff6b35' },
    { name: '研究机构', value: 18, color: '#4caf50' },
    { name: '个人贡献者', value: 5, color: '#9c27b0' }
  ]

  const monthlyContributions = [
    { month: '10月', contributions: 1250 },
    { month: '11月', contributions: 1456 },
    { month: '12月', contributions: 1678 },
    { month: '1月', contributions: 1523 }
  ]

  const getTypeLabel = (type: string) => {
    switch (type) {
      case 'validator': return '验证节点'
      case 'data_provider': return '数据提供商'
      case 'researcher': return '研究机构'
      default: return '未知'
    }
  }

  const getTypeColor = (type: string) => {
    switch (type) {
      case 'validator': return 'primary'
      case 'data_provider': return 'secondary'
      case 'researcher': return 'success'
      default: return 'default'
    }
  }

  return (
    <Box>
      <Typography variant="h4" component="h1" gutterBottom>
        贡献者
      </Typography>
      <Typography variant="body1" color="text.secondary" paragraph>
        查看网络数据贡献者和贡献统计
      </Typography>

      <Box sx={{ borderBottom: 1, borderColor: 'divider', mb: 3 }}>
        <Tabs value={tabValue} onChange={(_, newValue) => setTabValue(newValue)}>
          <Tab label="贡献者列表" />
          <Tab label="统计分析" />
          <Tab label="排行榜" />
        </Tabs>
      </Box>

      <TabPanel value={tabValue} index={0}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              活跃贡献者
            </Typography>
            <TableContainer component={Paper} sx={{ backgroundColor: 'transparent' }}>
              <Table>
                <TableHead>
                  <TableRow>
                    <TableCell>贡献者</TableCell>
                    <TableCell>类型</TableCell>
                    <TableCell align="right">贡献数</TableCell>
                    <TableCell align="right">质量评分</TableCell>
                    <TableCell align="right">信誉值</TableCell>
                    <TableCell>最后活跃</TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {contributorsData.map((contributor) => (
                    <TableRow key={contributor.address}>
                      <TableCell>
                        <Box display="flex" alignItems="center" gap={2}>
                          <Avatar sx={{ width: 32, height: 32 }}>
                            {contributor.displayName.charAt(0)}
                          </Avatar>
                          <Box>
                            <Typography variant="body2" fontWeight={500}>
                              {contributor.displayName}
                            </Typography>
                            <Typography variant="caption" color="text.secondary">
                              {contributor.address.slice(0, 8)}...{contributor.address.slice(-4)}
                            </Typography>
                          </Box>
                        </Box>
                      </TableCell>
                      <TableCell>
                        <Chip 
                          label={getTypeLabel(contributor.type)}
                          color={getTypeColor(contributor.type) as any}
                          size="small"
                        />
                      </TableCell>
                      <TableCell align="right">{contributor.contributions}</TableCell>
                      <TableCell align="right">{contributor.quality}%</TableCell>
                      <TableCell align="right">{contributor.reputation}</TableCell>
                      <TableCell>
                        {new Date(contributor.lastActive).toLocaleDateString()}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </CardContent>
        </Card>
      </TabPanel>

      <TabPanel value={tabValue} index={1}>
        <Grid container spacing={3}>
          <Grid item xs={12} md={6}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  贡献者类型分布
                </Typography>
                <ResponsiveContainer width="100%" height={300}>
                  <PieChart>
                    <Pie
                      data={contributionStats}
                      cx="50%"
                      cy="50%"
                      innerRadius={60}
                      outerRadius={120}
                      paddingAngle={5}
                      dataKey="value"
                    >
                      {contributionStats.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={entry.color} />
                      ))}
                    </Pie>
                    <Tooltip />
                  </PieChart>
                </ResponsiveContainer>
                <Box display="flex" flexWrap="wrap" gap={1} justifyContent="center" mt={2}>
                  {contributionStats.map((stat, index) => (
                    <Chip
                      key={index}
                      label={`${stat.name}: ${stat.value}`}
                      size="small"
                      sx={{ backgroundColor: stat.color, color: 'white' }}
                    />
                  ))}
                </Box>
              </CardContent>
            </Card>
          </Grid>

          <Grid item xs={12} md={6}>
            <Card>
              <CardContent>
                <Typography variant="h6" gutterBottom>
                  月度贡献趋势
                </Typography>
                <ResponsiveContainer width="100%" height={300}>
                  <BarChart data={monthlyContributions}>
                    <CartesianGrid strokeDasharray="3 3" opacity={0.3} />
                    <XAxis dataKey="month" />
                    <YAxis />
                    <Tooltip />
                    <Bar dataKey="contributions" fill="#00d4ff" />
                  </BarChart>
                </ResponsiveContainer>
              </CardContent>
            </Card>
          </Grid>
        </Grid>
      </TabPanel>

      <TabPanel value={tabValue} index={2}>
        <Card>
          <CardContent>
            <Typography variant="h6" gutterBottom>
              贡献排行榜
            </Typography>
            <Box>
              {contributorsData
                .sort((a, b) => b.contributions - a.contributions)
                .map((contributor, index) => (
                  <Box
                    key={contributor.address}
                    display="flex"
                    alignItems="center"
                    gap={2}
                    py={2}
                    borderBottom={index < contributorsData.length - 1 ? '1px solid rgba(255,255,255,0.1)' : 'none'}
                  >
                    <Typography
                      variant="h6"
                      sx={{
                        width: 32,
                        height: 32,
                        borderRadius: '50%',
                        backgroundColor: index === 0 ? '#ffd700' : index === 1 ? '#c0c0c0' : index === 2 ? '#cd7f32' : 'rgba(255,255,255,0.1)',
                        color: index < 3 ? '#000' : 'inherit',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                        fontSize: '14px',
                        fontWeight: 600
                      }}
                    >
                      {index + 1}
                    </Typography>
                    <Avatar sx={{ width: 40, height: 40 }}>
                      {contributor.displayName.charAt(0)}
                    </Avatar>
                    <Box flex={1}>
                      <Typography variant="body1" fontWeight={500}>
                        {contributor.displayName}
                      </Typography>
                      <Typography variant="body2" color="text.secondary">
                        {contributor.contributions} 次贡献 • 质量评分 {contributor.quality}%
                      </Typography>
                    </Box>
                    <Chip 
                      label={getTypeLabel(contributor.type)}
                      color={getTypeColor(contributor.type) as any}
                      size="small"
                    />
                  </Box>
                ))}
            </Box>
          </CardContent>
        </Card>
      </TabPanel>
    </Box>
  )
}

export default Contributors