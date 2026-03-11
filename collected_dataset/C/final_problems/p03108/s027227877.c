#include<stdio.h>
#include<string.h>
#include<math.h>
#include<stdlib.h>
#include<limits.h>

#define rep(i,begin,end) for(int i=begin; i<end; i++)
#define revrep(i,begin,end) for(int i=begin; i>end; i--)
#define lld long long int

void UF_init_set(int* set, int* size, int n ){
    for(int k=0; k<n; k++){
        *(set+k) = k;
        *(size+k)= 1;
    }
}

int UF_set_find( int* set, int i ){
    int r=i;
    while( *(set+r) != r ){
        *(set+r) = *(set + *(set + r));
        r = *(set + r);
    }
    return r;
}

void UF_set_union( int* set, int* size, int n, int i, int j ){
    int p, q, tmp;
    p = UF_set_find(set,i);
    q = UF_set_find(set,j);
    if( *(size + p) < *(size + q) ){
        tmp=p; p=q; q=tmp;
    }
    *(set + q)=p;
    *(size + p) += *(size + q);
}

int main(){
    int n, m;
    scanf("%d %d", &n, &m);
    int group[n], size[n];
    UF_init_set(group, size, n);
    int a[m], b[m];
    rep(i, 0, m){
        scanf("%d %d", &a[i], &b[i]);
        a[i]--; b[i]--;
    }
    int ans[m];
    ans[m-1] = n * (n-1) / 2;
    revrep(i, m-2, -1){
        /*rep(j, 0, n){
            printf("%d ", group[j]);
        }
        printf("\n");*/
        int p = UF_set_find(group, a[i+1]);
        int q = UF_set_find(group, b[i+1]);
        if(p == q){
            ans[i] = ans[i+1];
        }else{
            ans[i] = ans[i+1] - size[p] * size[q];
            UF_set_union(group, size, n, p, q);
        }
    }
    rep(i, 0, m){
        printf("%d\n", ans[i]);
    }

    return 0;
}