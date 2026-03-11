#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

int par[100000];

void turn(int x){
    if (par[x]!=x){
        turn(par[x]);
    }
    par[par[x]]=x;
    par[x]=x;
}

void unite(int x,int y){
    if (par[x]==x && par[y]==y){
        if (x>y){
            par[x]=y;
        } else {
            par[y]=x;
        }
    } else if (par[x]==x){
        par[x]=y;
    } else if (par[y]==y){
        par[y]=x;
    } else {
        if (x>y){
            turn(x);
            par[x]=y;
        } else {
            turn(y);
            par[y]=x;
        }
    }
}

void unite2(int x,int y){
    if (par[x]==x && par[y]==y){
        if (x<y){
            par[x]=y;
        } else {
            par[y]=x;
        }
    } else if (par[x]==x){
        par[x]=y;
    } else if (par[y]==y){
        par[y]=x;
    } else {
        if (x<y){
            turn(x);
            par[x]=y;
        } else {
            turn(y);
            par[y]=x;
        }
    }
}

int count(int x){
    if (par[x]==x){
        return 0;
    } else {
        return count(par[x])+1;
    }
}


int main(void){
    int n;
    scanf("%d",&n);
    int a[n-1],b[n-1];
    for (int i=0; i<n-1; i++){
        scanf("%d%d",&a[i],&b[i]);
        a[i]--;
        b[i]--;
    }
    
    for (int i=0; i<n; i++){
        par[i]=i;
    }
    for (int i=0; i<n-1; i++){
        unite(a[i],b[i]);
    }
    int cost[n];
    for (int i=0; i<n; i++){
        cost[i]=count(i);
    }
    for (int i=0; i<n; i++){
        par[i]=i;
    }
    for (int i=0; i<n-1; i++){
        unite2(a[i],b[i]);
    }
    int Fennec=0,Snuke=0;
    for (int i=0; i<n; i++){
        if (cost[i]<=count(i)){
            Fennec++;    
        } else {
            Snuke++;
        }
    }
    if (Fennec>Snuke){
        printf("Fennec\n");
    } else {
        printf("Snuke\n");
    }
	return 0 ;
}
