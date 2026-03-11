#include<stdio.h>

int main() {
    long n,m,k;
    scanf("%ld %ld %ld",&n,&m,&k);
    long arr1[n];
    long arr2[m];
    for (long i=0 ; i<n ;i++) {
        scanf("%ld",&arr1[i]);
    }
    for (long i=0 ; i<m ;i++) {
        scanf("%ld",&arr2[i]);
    }
    long i=0,j=0,rd=0;
    while ( k>=0 ) {
        if ( k==0 ) {
            rd++;
            break;
        }    
        else if ( rd%2 == 0 && i<n ) {
            k -= arr1[i];
            i++;
        }
        else if (j<m) {
            k -= arr2[j];
            j++;
        }   
        rd++;
    }
    printf("%ld",rd-1);
    return 0;
}