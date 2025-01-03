# -*- coding: utf-8 -*-
# Create Date: 2024/07/11
# Author: wangtao <wangtao.cpu@gmail.com>
# File Name: course_graph/parser/config.py
# Description: 定义知识图谱抽取配置

from dataclasses import dataclass, field


@dataclass
class Config:
    ignore_page: list[str] = field(default_factory=lambda: [])

config = Config()