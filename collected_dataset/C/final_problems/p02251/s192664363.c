/*
 * FileName:     change_making
 * CreatedDate:  2020-04-03 01:16:56 +0900
 * LastModified: 2020-04-03 01:22:51 +0900
 */

#include <stdio.h>

int main(void){
    int n,count=0;
    scanf("%d",&n);
    count+=n/25;
    int n_1=n-n/25*25;
    count+=n_1/10;
    int n_2=n_1-n_1/10*10;
    count+=n_2/5;
    int n_3=n_2-n_2/5*5;
    count+=n_3;
    printf("%d\n",count);
    return 0;
}

