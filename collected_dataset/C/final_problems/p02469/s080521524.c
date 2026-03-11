/*
 * FileName:     least_common_multiple_fix
 * CreatedDate:  2020-07-26 13:29:54 +0900
 * LastModified: 2020-07-26 13:41:11 +0900
 */

#include <stdio.h>
#include <stdlib.h>
long int max(long int s, long int t){
    if( s>t ) return s;
    return t;
}
long int min(long int s, long int t){
    if( s<t ) return s;
    return t;
}
long int GCD(long int s, long int t){
    if( s==0 ){
        return t;
    }
    else{
        return GCD( t%s, s);
    }
}
int main(void){
    int n;scanf("%d", &n);
    long int *a = malloc(n*sizeof(long int));
    for(int i=0; i<n; i++)scanf("%ld", a+i);
    long int gcd = 0, lcm = 0;
    for(int i=1; i<n; i++){
        if( i==1 ){
            gcd = GCD(min(a[i-1], a[i]), max(a[i-1], a[i]));
            lcm = a[i-1]*a[i] / gcd;
            continue;
        }
        gcd = GCD(min(lcm, a[i]), max(lcm, a[i]));
        lcm = a[i]*lcm / gcd;
    }
    printf("%ld\n", lcm);
    free(a);
    return 0;
}

