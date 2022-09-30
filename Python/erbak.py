# -*- coding: utf-8 -*-
import os
import json
import time
import shutil
ELDEN_RING_DIR = os.path.join(os.path.expanduser('~'), "AppData\Roaming\EldenRing")
BAKUP_DIR = r"D:\ZhaoPF\EldenRingBakup"


class Backup(object):
    def __init__(self, from_dir):
        self.func_list = ["退出", "创建新的备份", "创建新的备份覆盖现有备份", "还原备份覆盖当前记录", "删除备份", "查看备份"]
        self.from_dir = from_dir
        self.backup_dir = BAKUP_DIR
        self.backup_record = os.path.join(
            self.backup_dir, "backup_record.json")

    def print_red(self, s):
        print(f"\033[1;31m{s}\033[0m")

    def print_green(self, s):
        print(f"\033[1;32m{s}\033[0m")

    def print_yellow(self, s):
        print(f"\033[1;33m{s}\033[0m")

    def exists_check(self):
        if not os.path.isdir(self.from_dir):
            self.print_red("文件夹不存在！")
            exit()
        if not os.path.isdir(self.backup_dir):
            os.mkdir(self.backup_dir)
        if not os.path.isfile(self.backup_record):
            with open(self.backup_record, "w", encoding="utf8") as f:
                json.dump(dict(), f)

    def sort_dict_by_keys(self, d):
        keys = list(d.keys())
        keys.sort()
        new_dict = {}
        for key in keys:
            new_dict[key] = d[key]
        return new_dict

    def print_backup_record(self):
        with open(self.backup_record, "r", encoding="utf8") as f:
            backup_record = json.load(f)
        if not backup_record:
            self.print_yellow("当前备份记录为空！")
        else:
            print("------当前备份------")
            backup_record = self.sort_dict_by_keys(backup_record)
            for key in backup_record:
                print("%d.%s" % (int(key)-10000, backup_record[key]))
            print("--------------------")
        return backup_record

    def my_input(self, input_type, args=None):
        if input_type == "num":
            while True:
                if args[0] == "recovery":
                    i = input("请输入需要还原的记录编号：").strip()
                elif args[0] == "menu":
                    i = input("请输入菜单对应的选项编号：").strip()
                elif args[0] == "backup":
                    i = input("请输入新备份将会覆盖的记录编号：").strip()
                elif args[0] == "remove":
                    i = input("请输入需要删除的记录编号：").strip()
                else:
                    i = input("请输入选项：").strip()
                if i.isdigit():
                    return int(i)
                else:
                    self.print_red("请输入数字！")
                    continue
        elif input_type == "str":
            if args[0] == "record":
                s = input("请输入这条备份记录的名称（直接回车将会设置为当前的时间）：").strip()
                if not s:
                    s = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime())
                return s
        elif input_type == "bool":
            if args[0] == "recovery":
                while True:
                    s = input(
                        f"是否还原【编号：{args[1]}】【名称：{args[2]}】的备份到当前存档（Y/N）:").strip()
                    if s == "y" or s == "Y":
                        return True
                    elif s == "n" or s == "N":
                        return False
                    else:
                        self.print_red("请输入Y或者N进行确认!")
            elif args[0] == "backup":
                while True:
                    s = input(
                        f"是否创建新备份并覆盖到【编号：{args[1]}】【名称：{args[2]}】的备份（Y/N）:").strip()
                    if s == "y" or s == "Y":
                        return True
                    elif s == "n" or s == "N":
                        return False
                    else:
                        self.print_red("请输入Y或者N进行确认!")
            elif args[0] == "remove":
                while True:
                    s = input(
                        f"是否删除【编号：{args[1]}】【名称：{args[2]}】的备份（Y/N）:").strip()
                    if s == "y" or s == "Y":
                        return True
                    elif s == "n" or s == "N":
                        return False
                    else:
                        self.print_red("请输入Y或者N进行确认!")

    def 创建新的备份(self):
        backup_record = self.print_backup_record()
        backup_code = 10001
        while backup_code <= 19999:
            if str(backup_code) in backup_record.keys():
                backup_code += 1
                continue
            else:
                s = self.my_input("str", ["record", ])
                backup_record[str(backup_code)] = s
                shutil.copytree(self.from_dir, os.path.join(
                    self.backup_dir, str(backup_code)))
                with open(self.backup_record, "w", encoding="utf8") as f:
                    json.dump(backup_record, f)
                self.print_backup_record()
                self.print_green("新备份添加完成！")
                break
        else:
            self.print_red("已达到最大备份记录数量，请在代码修改相关限制！")

    def 创建新的备份覆盖现有备份(self):
        backup_record = self.print_backup_record()
        if not backup_record:
            return
        num = self.my_input("num", ["backup", ])
        key = str(num+10000)
        if key in backup_record.keys():
            check = self.my_input(
                "bool", ["backup", num, backup_record[key]])
            if check:
                shutil.rmtree(os.path.join(self.backup_dir, str(key)))
                shutil.copytree(self.from_dir, os.path.join(
                    self.backup_dir, str(key)))
                self.print_green("新备份已经创建并覆盖到【编号：%s】【名称：%s】的存档！" %
                                 (str(num), backup_record[key]))
            else:
                self.print_yellow("操作取消！")
        else:
            self.print_red("没有找到对应的记录！")

    def 还原备份覆盖当前记录(self):
        backup_record = self.print_backup_record()
        if not backup_record:
            return
        num = self.my_input("num", ["recovery", ])
        key = str(num+10000)
        if key in backup_record.keys():
            check = self.my_input(
                "bool", ["recovery", num, backup_record[key]])
            if check:
                shutil.rmtree(self.from_dir)
                shutil.copytree(os.path.join(
                    self.backup_dir, key), self.from_dir)
                self.print_green("【编号：%s】【名称：%s】的存档已经还原！" %
                                 (str(num), backup_record[key]))
            else:
                self.print_yellow("操作取消！")
        else:
            self.print_red("没有找到对应的记录！")

    def 删除备份(self):
        backup_record = self.print_backup_record()
        if not backup_record:
            return
        num = self.my_input("num", ["remove", ])
        key = str(num+10000)
        if key in backup_record.keys():
            check = self.my_input(
                "bool", ["remove", num, backup_record[key]])
            if check:
                try:
                    shutil.rmtree(os.path.join(self.backup_dir, key))
                except:
                    self.print_red("【编号：%s】【名称：%s】的存档文件删除失败！\n存档记录将会继续删除！" %
                                 (str(num), backup_record[key]))
                backup_record_remove = backup_record.copy()
                backup_record_remove.pop(key)
                with open(self.backup_record, "w", encoding="utf8") as f:
                    json.dump(backup_record_remove, f)
                self.print_green("【编号：%s】【名称：%s】的存档记录已删除！" %
                                 (str(num), backup_record[key]))
                
            else:
                self.print_yellow("操作取消！")
        else:
            self.print_red("没有找到对应的记录！")

    def 查看备份(self):
        self.print_backup_record()

    def run(self):
        print("--------菜单--------")
        for i, func in enumerate(self.func_list):
            print("%d.%s" % (i, func))
        print("--------------------")
        num = self.my_input("num", ["menu", ])
        if num < len(self.func_list):
            getattr(self, self.func_list[num])()
        else:
            self.print_red("没有找到对应的选项！")

    def 退出(self):
        self.print_red("Bye bye~")
        exit()


def main():
    print(f"-----艾尔登法环存档备份工具-----\n\
如需修改备份路径请修改对应的全局变量！\n\
源存档路径：{ELDEN_RING_DIR}\n\
备份路径：{BAKUP_DIR}\n")
    ed = Backup(ELDEN_RING_DIR)
    ed.exists_check()
    while True:
        ed.run()


if __name__ == "__main__":
    main()
