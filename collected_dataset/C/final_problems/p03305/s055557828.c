#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>
#include <math.h>
#include <limits.h>

int main(void){
    long long n,m,s,t;
    scanf("%lld%lld%lld%lld",&n,&m,&s,&t);
    s--;
    t--;
    long long u[m],v[m],a[m],b[m];
    for (int i=0; i<m; i++){
        scanf("%lld%lld%lld%lld",&u[i],&v[i],&a[i],&b[i]);
        u[i]--;
        v[i]--;
    }
    long long y[m],k[m];
    bool visited[n];
    for (int i=0; i<n; i++){
        y[i]=LLONG_MAX;
        visited[i]=false;
    }    
    y[s]=0;
    for (int i=0; i<n; i++){
        long long tmp=LLONG_MAX,next;
        for (int l=0; l<n; l++){
            if (tmp>y[l] && visited[l]==false){
                tmp=y[l];
                next=l;
            }
        }
        visited[next]=true;
        for (int l=0; l<m; l++){
            if (u[l]==next){
                if (y[v[l]]>y[next]+a[l]){
                    y[v[l]]=y[next]+a[l];
                }
            }
            if (v[l]==next){
                if (y[u[l]]>y[next]+a[l]){
                    y[u[l]]=y[next]+a[l];
                }
            }
        }
    }
    for (int i=0; i<n; i++){
        k[i]=LLONG_MAX;
        visited[i]=false;
    }    
    k[t]=0;
    for (int i=0; i<n; i++){
        long long tmp=LLONG_MAX,next;
        for (int l=0; l<n; l++){
            if (tmp>k[l] && visited[l]==false){
                tmp=k[l];
                next=l;
            }
        }
        visited[next]=true;
        for (int l=0; l<m; l++){
            if (u[l]==next){
                if (k[v[l]]>k[next]+b[l]){
                    k[v[l]]=k[next]+b[l];
                }
            }
            if (v[l]==next){
                if (k[u[l]]>k[next]+b[l]){
                    k[u[l]]=k[next]+b[l];
                }
            }
        }
    } 
    long long ans[n];
    for (int i=0; i<n; i++){
        ans[i]=y[i]+k[i];
    }
    for (int i=n-1; i>0; i--){
        if (ans[i-1]>ans[i]){
            ans[i-1]=ans[i];
        }
    }
    for (int i=0; i<n; i++){
        printf("%lld\n",1000000000000000-ans[i]);
    }
	return 0 ;
}