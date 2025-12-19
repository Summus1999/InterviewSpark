/**
 * Resume and Job Description templates for quick start
 */

export interface ResumeTemplate {
  id: string
  name: string
  industry: string
  position: string
  content: string
}

export interface JDTemplate {
  id: string
  name: string
  industry: string
  position: string
  content: string
}

// ===== Resume Templates =====

export const resumeTemplates: ResumeTemplate[] = [
  {
    id: 'frontend-3years',
    name: '前端开发 - 3年经验',
    industry: '互联网',
    position: '前端工程师',
    content: `姓名：张三
联系方式：138xxxx5678 | zhang.san@example.com
职位目标：前端工程师/高级前端工程师

教育背景：
大学本科 | 计算机科学与技术 | 2018-2022

技能技术栈：
- 前端框架：Vue.js 3、React、Angular
- 语言：JavaScript、TypeScript、HTML5、CSS3
- 构建工具：Webpack、Vite、Gulp
- 状态管理：Vuex、Pinia、Redux
- 其他技能：Git、RESTful API、性能优化、响应式设计

工作经历：
【ABC科技有限公司】前端开发工程师 | 2021.7 - 至今
- 负责电商平台PC端和移动端的前端开发与维护
- 使用Vue3+TypeScript完成核心业务模块开发
- 优化首屏加载速度，从4.5s降至1.2s，提升73%
- 参与UI组件库设计与开发，规范公司前端技术体系
- 主导前端构建工具升级，Webpack迁移至Vite，编译速度提升5倍

【XYZ公司】初级前端工程师 | 2020.3 - 2021.6
- 参与CRM系统前端开发
- 完成40+页面的功能实现与测试
- 与后端协作制定API规范

项目案例：
1. 电商商品详情页面 - Vue3、Vite、Element Plus
   在线访问：example.com/shop | GitHub：github.com/zhangsan/shop
2. 数据可视化仪表板 - React、ECharts、Ant Design
   说明：支持实时数据展示和交互分析

自我评价：
具有扎实的前端开发基础，熟悉现代前端开发流程，能够独立完成业务需求的分析、开发和测试。注重代码质量和用户体验，积极学习新技术。`
  },
  {
    id: 'backend-5years',
    name: '后端开发 - 5年经验',
    industry: '互联网',
    position: '后端工程师',
    content: `姓名：李四
联系方式：138xxxx1234 | lisi@example.com
职位目标：后端工程师/技术负责人

教育背景：
大学本科 | 软件工程 | 2016-2020

技能技术栈：
- 编程语言：Java、Python、Go、SQL
- 框架与库：Spring Boot、Django、Gin
- 数据库：MySQL、PostgreSQL、MongoDB、Redis
- 消息队列：RabbitMQ、Kafka
- 缓存技术：Redis、Memcached
- 中间件与工具：Docker、Kubernetes、Jenkins
- 微服务架构：ServiceMesh、API Gateway

工作经历：
【大型互联网公司】高级后端工程师 | 2019.1 - 至今
- 负责订单系统的设计与开发，日均订单量100万+
- 使用Spring Boot + MySQL + Redis构建高可用系统
- 实现秒杀活动支持QPS 10000+，成功承载双11大促
- 参与微服务架构设计，完成3个服务模块的拆分
- 优化数据库查询，平均响应时间从200ms降至50ms
- 组织技术分享，传授5名初级工程师

【中型科技公司】后端工程师 | 2017.6 - 2018.12
- 独立开发支付宝、微信支付接口
- 实现订单、发票、报表等核心模块
- 维护500万+用户数据的系统稳定性

项目案例：
1. 订单交易系统 - Java、Spring Boot、MySQL、Redis
   技术亮点：分布式事务、缓存一致性、秒杀设计
2. 实时消息推送系统 - Go、Kafka、Elasticsearch
   说明：支持100万并发连接，消息延迟<100ms

自我评价：
具有深厚的后端开发经验，熟悉互联网高并发场景，能够独立设计和优化复杂系统。有良好的系统思维和解决问题能力。`
  },
  {
    id: 'pm-2years',
    name: '产品经理 - 2年经验',
    industry: '互联网',
    position: '产品经理',
    content: `姓名：王五
联系方式：138xxxx9999 | wangwu@example.com
职位目标：产品经理/高级产品经理

教育背景：
大学本科 | 市场营销 | 2018-2022

专业技能：
- 产品设计：需求分析、产品规划、原型设计、竞品分析
- 数据分析：用户行为分析、转化漏斗、A/B测试
- 工具软件：Figma、Axure、SQL、Tableau、Jira
- 行业知识：电商、社交、内容、教育平台
- 软技能：跨部门沟通、团队协作、用户研究

工作经历：
【互联网公司】产品经理 | 2022.3 - 至今
- 负责用户中心模块，覆盖500万DAU
- 完成个人信息、账户安全等功能设计
- 推动用户实名认证率从45%提升至78%
- 主导"用户成长体系"从0-1设计，增加用户粘性25%
- 定期进行用户研究，完成15+深度访谈

【初创公司】产品实习生 | 2021.7 - 2022.2
- 参与内容社区APP产品迭代
- 完成20+feature的需求文档撰写
- 协助数据分析工作，发现关键流失环节

项目案例：
1. 用户成长体系设计 - 等级系统、勋章、积分
   效果：用户粘性提升25%，月活增长15%
2. 支付流程优化 - 简化步骤、优化转化
   效果：支付转化率从2.5%提升至3.2%
3. 新手引导重设计 - 用户路径分析、设计迭代
   效果：新用户留存率提升18%

自我评价：
具有从0-1的产品设计经验，数据敏感度高，能够平衡商业目标和用户体验。注重用户研究，有良好的文档和演讲能力。`
  },
  {
    id: 'fullstack-4years',
    name: '全栈开发 - 4年经验',
    industry: '互联网',
    position: '全栈工程师',
    content: `姓名：赵六
联系方式：138xxxx7777 | zhaoliu@example.com
职位目标：全栈工程师/技术负责人

教育背景：
大学本科 | 计算机科学与技术 | 2017-2021

技能技术栈：
前端：React、Vue3、TypeScript、Webpack、Vite、Tailwind CSS
后端：Node.js、Express、Python、Django
数据库：PostgreSQL、MongoDB、Redis
工具：Docker、Git、Linux、AWS
其他：RESTful API设计、GraphQL、WebSocket、微服务

工作经历：
【创业公司】全栈技术负责人 | 2021.1 - 至今
- 带领3人技术团队，从0-1构建SaaS产品
- 前后端架构设计，独立完成核心模块开发
- 用户从0增长至5000+，月营收20万+
- 实现自动化部署，从30分钟降至5分钟
- 建立代码规范和开发流程，代码质量评分A

【中等规模公司】高级工程师 | 2018.6 - 2020.12
- 同时负责React前端和Node.js后端开发
- 完成电商平台核心交易链路
- 日均订单100万+，系统可用性99.9%

项目案例：
1. SaaS企业管理系统 - React + Node.js + PostgreSQL
   功能：员工管理、考勤、工资、报表
   用户：500+企业，5000+员工
2. 内容发布平台 - Vue3 + Python + MongoDB
   功能：博客、评论、推荐系统
   效果：日均PV 100万，用户增长50%/月

自我评价：
既有深度的前端技能，也有扎实的后端基础，能够独立设计和实现复杂系统。创业经验丰富，能够快速迭代产品。`
  },
  {
    id: 'qa-3years',
    name: 'QA测试 - 3年经验',
    industry: '互联网',
    position: '测试工程师',
    content: `姓名：孙七
联系方式：138xxxx6666 | sunqi@example.com
职位目标：测试工程师/测试负责人

教育背景：
大学本科 | 计算机科学与技术 | 2018-2022

技能技术栈：
- 测试类型：功能测试、性能测试、自动化测试、接口测试
- 测试工具：Selenium、JMeter、Postman、Appium
- 编程语言：Python、Java、SQL
- CI/CD：Jenkins、GitLab CI
- 缺陷管理：Jira、禅道
- 其他技能：TestNG、Pytest、Mock、Linux基础

工作经历：
【互联网公司】QA工程师 | 2021.3 - 至今
- 负责电商平台PC端功能测试
- 建立自动化测试框架，覆盖率从0%提升至45%
- 开发性能测试脚本，支持5000并发压力测试
- 组织内部测试培训，提升团队效率20%
- 独立测试300+个feature，发现并记录bug 500+

【初创公司】QA实习生 | 2020.6 - 2021.2
- 参与移动APP功能和UI测试
- 编写30+条自动化测试用例
- 学习Appium框架

项目案例：
1. 自动化测试框架建设 - Selenium + Python + Jenkins
   成果：覆盖电商核心流程，执行时间从4h降至30min
2. 性能测试优化 - JMeter + 脚本优化
   成果：定位数据库瓶颈，优化后响应时间降50%
3. 接口自动化 - Postman + Python
   成果：支持CI集成，每次构建自动执行500+用例

自我评价：
具有全面的测试技能，特别是自动化测试经验丰富。能够独立分析问题、设计测试方案，对质量有高要求。`
  },
  {
    id: 'devops-4years',
    name: 'DevOps工程师 - 4年经验',
    industry: '互联网',
    position: 'DevOps工程师',
    content: `姓名：周八
联系方式：138xxxx8888 | zhoubo@example.com
职位目标：DevOps工程师/基础设施负责人

教育背景：
大学本科 | 网络工程 | 2016-2020

技能技术栈：
- 云平台：AWS、阿里云、腾讯云
- 容器化：Docker、Kubernetes、Helm
- CI/CD：Jenkins、GitLab CI、ArgoCD
- 配置管理：Ansible、Terraform
- 监控告警：Prometheus、Grafana、ELK
- 编程语言：Bash、Python、Go
- Linux系统：网络、存储、性能优化

工作经历：
【大型互联网公司】DevOps工程师 | 2020.1 - 至今
- 负责公司基础设施和容器编排
- 搭建Kubernetes集群，支持100+微服务
- 完善CI/CD流程，部署从1h降至5min
- 实现全链路监控，故障告警时间<1min
- 建立灾难恢复方案，RTO<30min，RPO<5min
- 指导5名初级工程师

【中型公司】运维工程师 | 2018.6 - 2019.12
- 管理50台服务器基础设施
- 实现自动化部署脚本
- 日志分析和性能优化

项目案例：
1. Kubernetes集群建设 - 自建K8s + 云原生
   规模：100+节点，1000+pod，99.99%可用性
2. CI/CD流程优化 - Jenkins + Docker + GitLab
   效果：部署时间从60分钟降至5分钟
3. 监控告警体系 - Prometheus + Grafana + AlertManager
   覆盖：服务、主机、业务等多维度监控

自我评价：
具有丰富的云平台和Kubernetes经验，能够设计和管理大规模基础设施。代码能力强，能够编写高效的自动化脚本。`
  }
]

// ===== Job Description Templates =====

export const jdTemplates: JDTemplate[] = [
  {
    id: 'frontend-senior',
    name: '高级前端工程师',
    industry: '互联网',
    position: '前端工程师',
    content: `职位名称：高级前端工程师
工作地点：北京/上海/深圳
薪资范围：25K-40K

岗位职责：
1. 负责公司核心产品前端开发和维护
2. 参与前端技术方案设计和架构优化
3. 制定和推行前端开发规范和最佳实践
4. 指导初级工程师，进行代码审查和技术分享
5. 性能优化、浏览器兼容性等技术问题解决
6. 参与用户体验改进，优化页面加载速度和交互

任职要求：
1. 3年以上前端开发经验，1年以上高级开发经验
2. 精通Vue.js和React框架，理解框架底层原理
3. 掌握TypeScript、ES6+、函数式编程
4. 熟悉Webpack、Vite等构建工具，能够优化打包体积
5. 掌握浏览器调试、性能分析工具
6. 有微前端、组件库或可视化项目经验优先
7. 良好的代码规范和团队协作能力

我们期待你：
- 具有较强的技术深度和广度
- 对用户体验有追求
- 能够独立推动技术创新
- 愿意指导和帮助他人成长`
  },
  {
    id: 'backend-senior',
    name: '高级后端工程师',
    industry: '互联网',
    position: '后端工程师',
    content: `职位名称：高级后端工程师
工作地点：北京/上海/深圳
薪资范围：30K-50K

岗位职责：
1. 负责后端系统的架构设计和实现
2. 处理高并发、大数据量的技术挑战
3. 进行系统性能优化和成本控制
4. 参与技术选型和新技术评估
5. 组织技术分享，提升团队技术水平
6. 参与系统容量规划和故障应急

任职要求：
1. 5年以上后端开发经验，2年以上高级工程师经验
2. 精通Java/Go/Python，掌握多种编程范式
3. 深入理解数据库原理，有调优经验
4. 有分布式系统、微服务架构设计经验
5. 掌握Redis、消息队列等中间件
6. 了解容器化和Kubernetes基础
7. 有高并发系统设计经验优先

我们期待你：
- 有互联网大厂工作经验
- 能够设计高可用、高性能系统
- 具有技术前瞻性和创新意识
- 有团队管理或带人经验优先`
  },
  {
    id: 'pm-mid-level',
    name: '中级产品经理',
    industry: '互联网',
    position: '产品经理',
    content: `职位名称：产品经理（中级）
工作地点：北京/上海/杭州
薪资范围：18K-28K

岗位职责：
1. 负责产品模块的需求分析、设计和上线
2. 进行用户研究，理解用户需求
3. 驱动产品功能从conception到上线的全过程
4. 分析产品数据，优化用户体验
5. 与设计、开发、运营等部门紧密协作
6. 进行竞品分析，提出优化建议
7. 撰写产品需求文档和原型

任职要求：
1. 2年以上互联网产品经理工作经验
2. 了解用户研究方法，能进行用户访谈
3. 掌握Axure、Figma等原型工具
4. 具有数据分析能力，能使用SQL进行数据查询
5. 了解常见互联网产品形态（电商、社交、内容等）
6. 强的逻辑思维和表达能力
7. 能够独立推动一个功能模块的完整生命周期

我们期待你：
- 对产品设计有热情
- 用户同理心强
- 能够快速学习新领域知识
- 有初创公司或创新项目经验优先`
  },
  {
    id: 'qa-mid-level',
    name: '测试工程师（自动化方向）',
    industry: '互联网',
    position: '测试工程师',
    content: `职位名称：测试工程师（自动化方向）
工作地点：北京/上海/深圳
薪资范围：15K-25K

岗位职责：
1. 负责产品功能、性能和兼容性测试
2. 设计和开发自动化测试框架和用例
3. 建立持续集成的测试流程
4. 进行性能和压力测试
5. 分析和定位bug根本原因
6. 参与需求评审和用户反馈分析
7. 编写测试报告和质量分析

任职要求：
1. 2年以上功能测试经验，1年以上自动化测试经验
2. 掌握Python/Java编程
3. 熟悉Selenium、Appium等自动化工具
4. 了解SQL语言，能进行数据库测试
5. 掌握JMeter等性能测试工具
6. 熟悉Linux基础命令
7. 有CI/CD集成经验优先

我们期待你：
- 对质量有高要求
- 具有发散思维，能思考边界测试场景
- 能够独立分析和解决问题
- 有测试框架开发或技术深化经验优先`
  },
  {
    id: 'devops-mid-level',
    name: 'DevOps工程师',
    industry: '互联网',
    position: 'DevOps工程师',
    content: `职位名称：DevOps工程师
工作地点：北京/上海/杭州
薪资范围：20K-35K

岗位职责：
1. 负责公司基础设施和运维系统的设计与维护
2. 管理云平台资源，优化成本
3. 设计和实现CI/CD流程
4. 维护Kubernetes等容器编排平台
5. 建立监控告警体系
6. 处理故障应急和恢复
7. 参与系统容量规划

任职要求：
1. 3年以上运维或DevOps工作经验
2. 精通Linux系统管理
3. 掌握Docker和Kubernetes
4. 熟悉至少一个云平台（AWS/阿里云/腾讯云）
5. 掌握至少一种编程语言（Python/Go/Bash）
6. 熟悉Jenkins或GitLab CI等CI/CD工具
7. 有Terraform或Ansible配置管理经验

我们期待你：
- 有大型系统运维经验
- 能够设计高可用、高可靠的基础设施
- 具有较强的自动化能力
- 有安全和成本优化意识`
  },
  {
    id: 'mobile-ios',
    name: 'iOS开发工程师',
    industry: '互联网',
    position: 'iOS工程师',
    content: `职位名称：iOS开发工程师
工作地点：北京/上海/深圳
薪资范围：18K-32K

岗位职责：
1. 负责iOS应用的设计、开发和维护
2. 优化应用性能，降低包体积和内存占用
3. 参与技术方案评审和架构设计
4. 进行代码审查和技术分享
5. 适配不同iOS版本和设备
6. 与UI设计师紧密协作
7. 参与上线前的各类测试

任职要求：
1. 3年以上iOS开发经验
2. 深入理解Objective-C或Swift语言
3. 掌握iOS开发框架（UIKit/SwiftUI）
4. 了解内存管理、多线程、网络通信
5. 有第三方SDK集成经验
6. 了解iOS应用签名和上架流程
7. 熟悉Git、Xcode等开发工具

我们期待你：
- 代码质量和用户体验有追求
- 能够独立完成一个功能模块
- 积极学习新技术和行业动向
- 有开源项目或个人APP经验优先`
  }
]

/**
 * Helper functions for template management
 */

export function getResumeTemplatesByIndustry(industry: string): ResumeTemplate[] {
  return resumeTemplates.filter(t => t.industry === industry)
}

export function getJDTemplatesByIndustry(industry: string): JDTemplate[] {
  return jdTemplates.filter(t => t.industry === industry)
}

export function getIndustries(): string[] {
  const industries = new Set<string>()
  resumeTemplates.forEach(t => industries.add(t.industry))
  jdTemplates.forEach(t => industries.add(t.industry))
  return Array.from(industries)
}
