#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <math.h>

#define LINE_BUF_SIZE 100

int main(){

    int i,j,k;
    int max = 0;
    int count = 0;
    int M,N,v;
    char line[LINE_BUF_SIZE];

    fgets(line,sizeof(line),stdin);
    sscanf(line,"%d", &N);

    long int A[N+2];

    A[0] = 0;
    A[N+1] = 0;

    for(i=1;i<=N;i++){
        scanf("%d",&v);
        A[i] = v;
    }

    long int ans = 0;

    for(i=0;i<=N;i++){              /*本来の金額*/
        ans += fabs(A[i+1]-A[i]);
    }

    int ans_hold = ans;

    for(i=1;i<=N;i++){   /*最後は残す*/

        if(A[i+1] > A[i] && A[i] > A[i-1]){
            printf("%d\n",ans);
        }

        else{
            ans = ans - (fabs(A[i-1]-A[i]) + fabs(A[i+1]-A[i])) + fabs(A[i+1]-A[i-1]);
            printf("%d\n",ans);
            ans  = ans_hold;
        }
    }

    return 0;

}
