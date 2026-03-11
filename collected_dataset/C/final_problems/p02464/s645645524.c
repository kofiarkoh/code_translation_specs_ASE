/*
 * FileName:     set_intersection
 * CreatedDate:  2020-06-09 16:10:58 +0900
 * LastModified: 2020-06-09 16:14:46 +0900
 */

#include <stdio.h>
#include <stdlib.h>
#define ultra 2e9
int main(void){
    int n;scanf("%d",&n);
    int *a = malloc((n+1)*sizeof(int));
    for(int i=0;i<n;i++)scanf("%d",a+i);
    a[n]=ultra;
    int m;scanf("%d",&m);
    int *b = malloc((m+1)*sizeof(int));
    for(int i=0;i<m;i++)scanf("%d",b+i);
    b[m]=ultra;
    int ia=0,ib=0;
    while(ia<n||ib<m){
        if(a[ia]<b[ib]){
            ia++;
        }
        else if(a[ia]>b[ib]){
            ib++;
        }
        else{
            printf("%d\n",a[ia]);
            ia++;
            ib++;
        }
    }
    free(a);
    free(b);
    return 0;
}

