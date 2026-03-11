/*
 * FileName:     unique
 * CreatedDate:  2020-06-09 13:55:12 +0900
 * LastModified: 2020-06-09 14:00:33 +0900
 */

#include <stdio.h>
#include <stdlib.h>
int main(void){
    int n;scanf("%d",&n);
    int *a = malloc(n*sizeof(int));
    int *box = malloc(n*sizeof(int)),count=0;
    for(int i=0;i<n;i++){
        scanf("%d",a+i);
        if(i==0){
            box[count++]=a[0];
        }
        if(i>0){
            if(a[i]!=a[i-1]){
                box[count++]=a[i];
            }
        }
    }
    printf("%d",a[0]);
    for(int i=1;i<count;i++)printf(" %d",box[i]);
    printf("\n");
    return 0;
}

