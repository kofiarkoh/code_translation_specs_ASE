//
//  main.c
//  Osaki
//
//  Created by x15071xx on 2016/06/21.
//  Copyright ?? 2016??´ AIT. All rights reserved.
//

#include <stdio.h>
#define DAY 86400

void swap(int time[10000],int j,int k){
    int tmp;
    
    tmp = time[j];
    time[j] = time[k];
    time[k] = tmp;
    
}

int main(int argc, const char * argv[]) {
    
    int i,j,k;
//    scanf("%d %d",&i,&j);
//    int ii,jj,kk;
//    for (i=0; i<1000; i++) {
//        scanf("%d:%d:%d %d:%d:%d",&i,&j,&k,&ii,&jj,&kk);
//        printf("%d %d %d %d %d %d\n",i,j,k,ii,jj,kk);
//    }
//    
    while (1) {
        
        int count=0,count2=0;
        int n=0;
        int h=0,m=0,s=0;
        int hh=0,mm=0,ss=0;
        int train_num=0;
        int stock=0;
        int time[2][10000];//?????? ??°???
        
        for (i=0; i<2; i++) {
            for (j=0; j<10000; j++) {
                time[i][j]=0;
            }
        }
        scanf("%d",&n);
        
        if (n==0) {
//            for (i=0; i<=count; i++) {
//                printf("%d\n",num[i]);
//            }
            break;
        }
        
        for (i=0; i<n; i++) {
            scanf("%d:%d:%d",&h,&m,&s);
            scanf("%d:%d:%d",&hh,&mm,&ss);
            //???????¨????
            time[0][i] = h*3600 + m*60 + s;
            time[1][i] = hh*3600 + mm*60 + s;
            
            //printf("%d:%d %d\n",i,time[0][i],time[1][i]);
        }
        
        
        for (i=0; i<n-1; i++) {
            for (j=n-1; j>i; j--) {
                if (time[1][j] < time[1][j-1]) {
                    swap(time[1],j,j-1);
                }
            }
        }
        
        for (i=0; i<n-1; i++) {
            for (j=n-1; j>i; j--) {
                if (time[0][j] < time[0][j-1]) {
                    swap(time[0],j,j-1);
                }
            }
        }

        
//        for (i=0; i<n; i++) {
//            printf("%d %d\n",time[0][i],time[1][i]);
//        }
//        printf("\n\n");
        int max=0;
        while (count != n-1) {
            if (time[0][count] >= time[1][count2]) {
               
                if (count2 == n-1) {
                    time[1][count2] += 999999;
                } else {
                    count2++;
                }
                stock--;
               // printf("??°???%d:%d",count2,time[1][count2]);
               // printf(" %d %d\n",train_num,stock);
            } else if (time[0][count] < time[1][count2]) {
                
                if (count == n-1) {
                    time[0][count] += 999999;
                } else {
                    count++;
                }
                 stock++;
                if (max < stock) {
                    max = stock;
                }
               // printf("??????%d:%d",count,time[0][count]);
               // printf(" %d %d\n",train_num,stock);
            }
        }
        
        printf("%d\n",max);
        
    }
    
//    for (i=0; i<num_count; i++) {
//        printf("%d\n",num[i]);
//    }
    
    return 0;
}