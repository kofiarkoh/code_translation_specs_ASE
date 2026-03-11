#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

int parr[200000],part[200000];

int findr(int x){
    if (parr[x]==x){
        return x;
    } else {
        return parr[x]=findr(parr[x]);
    }
}

void uniter(int x, int y){
    x=findr(x);
    y=findr(y);
    if(x==y){
        return ;
    }
    parr[x]=y;
}

int findt(int x){
    if (part[x]==x){
        return x;
    } else {
        return part[x]=findt(part[x]);
    }
}

void unitet(int x, int y){
    x=findt(x);
    y=findt(y);
    if(x==y){
        return ;
    }
    part[x]=y;
}

int main(void){
    int n,k,l,p,q,r,s;
    scanf("%d%d%d",&n,&k,&l);
    
    for (int i=0; i<n; i++){
        parr[i]=i;
        part[i]=i;
    }
    
    for (int i=0; i<k; i++){
        scanf("%d%d",&p,&q);
        p--;
        q--;
        uniter(p,q);
    }
    
    for (int i=0; i<l; i++){
        scanf("%d%d",&r,&s);
        r--;
        s--;
        unitet(r,s);
    }
    
    int ra[n],t[n],ans[n];
    for (int i=0; i<n; i++){
        ra[i]=findr(i);
        t[i]=findt(i);
        ans[i]=1;
    }
    for (int i=0; i<n; i++){
        for (int j=i+1; j<n; j++){
            if (ra[i]==ra[j] && t[i]==t[j]){
                ans[i]++;
                ans[j]++;
            }
        }
        printf("%d ",ans[i]);
    }
    printf("\n");

	return 0 ;
}