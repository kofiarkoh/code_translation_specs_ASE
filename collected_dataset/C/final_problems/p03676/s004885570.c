//set many funcs
#include<stdio.h>
#define mod 1000000007

int main(void){
    long long x,y,n,i,j,k,a[131072],r,p[131072]={0},pd[16384];
    long long bd[16384]={0},md[16384]={0};
    long long bdm[131072]={0};
    long long cl[131072]={0},nbx;
    x=1;
    for(i=2;i<=131071;i++){
        if(p[i]==0){
            pd[x]=i;x++;
            for(j=2*i;j<=131071;j+=i){p[j]=1;}
        }
    }
    scanf("%lld",&n);n++;
    for(i = 1;i <= n;i++){scanf("%lld",&a[i]);}
    for(i=1;i<=n;i++){
        if(cl[(a[i])] == 0){cl[(a[i])] = i;}
        else{nbx=i-cl[(a[i])];nbx++;nbx=n-nbx;}
    }
    for(i=1;i<=n;i++){
        x=n+1-i;y=i;
        for(j=1;j<12000;j++){
            while(x%pd[j]==0){bd[j]++;x/=pd[j];}
            while(y%pd[j]==0){bd[j]--;y/=pd[j];}
            if(x==1 && y==1){break;}
        }
        if(i==1){printf("%lld\n",n-1);}
        else if(i <= nbx+1){
            x=nbx+2-i;y=i-1;
            for(j=1;j<12000;j++){
                while(x%pd[j]==0){md[j]++;x/=pd[j];}
                while(y%pd[j]==0){md[j]--;y/=pd[j];}
                if(x==1&&y==1){break;}
            }
            x=1;y=1;
            for(j=1;j<10500;j++){
                for(k=1;k<=bd[j];k++){x*=pd[j];x%=mod;}
                for(k=1;k<=md[j];k++){y*=pd[j];y%=mod;}
            }
            if(x<y){printf("%lld\n",mod+x-y);}
            else{printf("%lld\n",x-y);}
        }
        else{
            x=1;
            for(j=1;j<10500;j++){
                for(k=1;k<=bd[j];k++){x*=pd[j];x%=mod;}
            }
            printf("%lld\n",x);
        }
    }
    return 0;
}