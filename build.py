#! /usr/bin/python

from sys import argv
import os
import subprocess

def build_sample():
    subprocess.call(['cp', './target/debug/libsample.dylib', './platform/cache'])

def build_platform():
    os.chdir(os.curdir + '/platform')
    subprocess.call(['cargo', 'run'])

def clean():
    os.chdir(os.curdir + '/platform/cache')
    subprocess.call(['rm', 'libsample.dylib'])

def parse_args():
    if len(argv) != 2:
        print "Not enough arguments"
        exit(1)

    if argv[1] == 'sample':
        print "Gen lib to platform"
        build_sample()

    if argv[1] == 'platform':
        print "Run platform"
        build_platform()
    
    if argv[1] == 'clean':
        print 'Clean up gen lib in platform folder'
        clean()
    
    else:
        exit(1)
    
if __name__ == '__main__':
    parse_args()
