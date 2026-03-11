/*
 * FileName:     E
 * CreatedDate:  2020-06-13 22:13:22 +0900
 * LastModified: 2020-06-13 22:27:27 +0900
 */

#include <stdio.h>
#include <stdlib.h>
int main(void){
    int n,k;
    long int s,t;
    scanf("%d%d%ld%ld",&n,&k,&s,&t);
    long int *a = malloc(n*sizeof(long int));
    int sum=0;
    for(int i=0;i<n;i++)scanf("%ld",a+i);
    for(int i=0;i<1<<n;i++){
        int count=0;
        int seki=0,wa=0;
        for(int j=0;j<n;j++){
            if(1&i>>j){
                count++;
                wa=wa|a[j];
                if(count==1){
                    seki=a[j];
                }
                else{
                    seki=seki&a[j];
                }
            }
        }
        if(count<=k){
            if(s==seki&&t==wa){
                sum++;
            }
        }
    }
    printf("%d\n",sum);
    free(a);
    return 0;
}
