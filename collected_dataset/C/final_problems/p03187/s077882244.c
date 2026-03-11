#include <stdio.h>
#include <stdlib.h>

int main() {
    long L,N,*X,i,j,k;
    long long cr,cl;

    scanf("%ld %ld",&L,&N);

    X=(long *)malloc(N*sizeof(long));
    for(i=0;i<N;i++) {
        scanf("%ld",X+i);
    }

    cr=X[0];
    k=X[0];
    i=1;
    j=N-1;

    if(N!=1) {
        while(X[i]-k>=L-X[j]+k) {
            cr+=X[i]-k;
            k=X[i];
            i++;
            if(i==N-1) break;
        }
    }
    while(i<=j) {
        cr+=L-X[j]+k;
        k=X[j]-L;
        j--;
        if(i>j) break;
        cr+=X[i]-k;
        k=X[i];
        i++;
    }

    cl=L-X[N-1];
    k=X[N-1]-L;
    i=0;
    j=N-2;

    if(N!=1) {
        while(X[0]-k<=L-X[j]+k) {
            cl+=L-X[j]+k;
            k=X[j]-L;
            j--;
            if(j==-1) break;
        }
    }
    while(i<=j) {
        cl+=X[i]-k;
        k=X[i];
        i++;
        if(i>j) break;
        cl+=L-X[j]+k;
        k=X[j]-L;
        j--;
    }

    if(cr<cl) cr=cl;
    printf("%lld\n",cr);

    return 0;
}