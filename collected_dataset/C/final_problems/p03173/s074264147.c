#include<stdio.h>

int main() {
        int n;
        scanf("%d", &n);
        long long a[n], dp[n][n];
        for( int i=0; i<n; ++i )
                scanf("%lld",a+i), dp[i][i] = 0;
        for( int d=1; d<n; ++d )
                for( int l=0; l<=n-1-d; ++l )
                {
                        dp[l][l+d] = 1000000000000000000;
                        for( int m=l; m<l+d; ++m )
                                if( dp[l][m] + dp[m+1][l+d] < dp[l][l+d] )
                                        dp[l][l+d] = dp[l][m] + dp[m+1][l+d];
                        for( int m=l; m<=l+d; ++m )
                                dp[l][l+d] += a[m];
                }
        printf("%lld\n", dp[0][n-1] );
}
