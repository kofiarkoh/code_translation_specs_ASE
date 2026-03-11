//
//  main.c
//  Minor Change
//
//  Created by k19031kk on 2020/06/27.
//  Copyright © 2020 AIT. All rights reserved.
//

#include <stdio.h>
#include <string.h>

int main(int argc, const char * argv[]) {
    
    int ret,ret1;
    
    char sentence1[254];
    char sentence2[254];
    
    int count = 0;
    
    ret = scanf("%s",sentence1);
    
    if(ret == EOF){
        return -1;
    }
    
    ret1 = scanf("%s",sentence2);
    
    if(ret1 == EOF){
        return -1;
    }
    
    int len = (int) strlen(sentence1);
    int len1 = (int) strlen(sentence2);
    if(len != len1){
        return -1;
    }
    
//    for(int i=0;i<len;i++){
//        if(!(sentence1[i]>= 'a') && (sentence1[i]<='z')){
//            return -1;
//        }
//
//        if(!(sentence2[i]>= 'a') && (sentence2[i]<='z')){
//            return -1;
//        }
//    }
//
//
//    for(int i=0;i<len;i++){
//
//        if(sentence1[i] != sentence2[i]){
//            count++;
//        }
//    }
    
    for(int i=0;i<len;i++){
        if(sentence1[i]!=sentence2[i]){
            count++;
        }
    }
    
    printf("%d\n",count);
    return 0;
}
