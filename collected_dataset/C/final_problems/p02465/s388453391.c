/*
 * FileName:     set_difference
 * CreatedDate:  2020-06-09 16:18:09 +0900
 * LastModified: 2020-06-09 16:27:01 +0900
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
            printf("%d\n",a[ia]);
            ia++;
        }
        else if(a[ia]>b[ib]){
//            printf("%d\n",b[ib]);
            ib++;
        }
        else{
            ia++;
            ib++;
        }
    }
    free(a);
    free(b);
    return 0;
}

