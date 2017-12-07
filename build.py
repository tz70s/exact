#! /usr/bin/python

from sys import argv
import os
import subprocess

def build_sample():
    os.chdir(os.curdir + '/sample')
    subprocess.call(['cargo', 'build', '--release'])
    subprocess.call(['cp', './target/release/libsample.dylib', '../invoker'])

def build_invoker():
    os.chdir(os.curdir + '/invoker')
    subprocess.call(['cargo', 'run'])

def build_lib():
    os.chdir(os.curdir + '/actor-lib')
    subprocess.call(['cargo', 'build'])

def parse_args():
    if len(argv) != 2:
        print "Not enough arguments"
        exit(1)

    if argv[1] == 'sample':
        print "Build sample and gen lib to invoker"
        build_sample()

    if argv[1] == 'invoker':
        print "Build invoker"
        build_invoker()

    if argv[1] == 'lib':
        print "Build lib"
        build_lib()

    else:
        exit(1)
    
if __name__ == '__main__':
    parse_args()