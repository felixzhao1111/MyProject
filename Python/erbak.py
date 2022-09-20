# -*- coding: utf-8 -*-
import os
import json
import time
import shutil

BAKUP_DIR = r"D:\ZhaoPF\Bakup"

class Print(object):
    def red(self, s):
        print(f"\033[1;31m{s}\033[0m")
    def green(self, s):
        print(f"\033[1;32m{s}\033[0m")


class Backup(object):
    def __init__(self, from_dir):
        self.func_list = ["退出", "创建新的备份", "还原备份覆盖当前记录"]
        self.from_dir = from_dir
        self.backup_dir = BAKUP_DIR
        self.backup_record = os.path.join(
            self.backup_dir, "backup_record.json")
        self.print = Print()

    def exists_check(self):
        if not os.path.isdir(self.from_dir):
            self.print.red("文件夹不存在！")
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
            self.print.red("当前备份记录为空！")
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
                if args == "recovery":
                    i = input("请输入需要还原的记录编号：").strip()
                elif args == "menu":
                    i = input("请输入菜单对应的选项编号：").strip()
                else:
                    i = input("请输入选项：").strip()
                if i.isdigit():
                    return int(i)
                else:
                    self.print.red("请输入数字！")
                    continue
        elif input_type == "str":
            if args == "record":
                s = input("请输入这条备份记录的名称（直接回车将会设置为当前的时间）：").strip()
                if not s:
                    s = time.strftime("%Y-%m-%d %H:%M:%S", time.localtime())
                return s

    def 创建新的备份(self):
        backup_record = self.print_backup_record()
        backup_code = 10001
        while backup_code <= 19999:
            if str(backup_code) in backup_record.keys():
                backup_code += 1
                continue
            else:
                s = self.my_input("str", "record")
                backup_record[str(backup_code)] = s
                shutil.copytree(self.from_dir, os.path.join(self.backup_dir, str(backup_code)))
                with open(self.backup_record, "w", encoding="utf8") as f:
                    json.dump(backup_record, f)
                self.print.green("新备份添加完成！")
                self.print_backup_record()
                break
        else:
            self.print.red("已达到最大备份记录数量，请在代码修改相关限制！")

    def 还原备份覆盖当前记录(self):
        backup_record = self.print_backup_record()
        num = self.my_input("num", "recovery")
        for key in backup_record:
            if num == int(key)-10000:
                print(key, backup_record[key])
                break
        else:
            self.print.red("没有找到对应的记录！")
        

    def run(self):
        print("--------菜单--------")
        for i, func in enumerate(self.func_list):
            print("%d.%s" % (i, func))
        print("--------------------")
        num = self.my_input("num", "menu")
        if num < len(self.func_list):
            getattr(self, self.func_list[num])()
        else:
            self.print.red("没有找到对应的选项！")

    def 退出(self):
        self.print.red("Bye bye~")
        exit()


def main():
    ed = Backup(os.path.join(os.path.expanduser('~'), "ed"))
    ed.exists_check()
    while True:
        ed.run()


if __name__ == "__main__":
    main()
