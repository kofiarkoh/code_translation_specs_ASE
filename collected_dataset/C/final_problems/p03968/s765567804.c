#include<stdio.h>

long long calcce(long long pt[]){
  if(pt[0]==pt[1]&&pt[1]==pt[2]&&pt[2]==pt[3]){return 4;}
  if(pt[0]==pt[2]&&pt[1]==pt[3]){return 2;}
  return 1;
}

long long ispt(long long a[],long long b[]){
  long long i,j;
  for(i=0;i<4;i++){
    for(j=0;j<4;j++){
      if(a[j]!=b[(i+j)%4]){break;}
      if(j==3){return 1;}
    }
  }
  return 0;
}

long long calc(long long cnt[]){
  long long i,j,k,l,res=0,dt;
  long long pj,pk,pl;
  //for(i=0;i<16;i++){printf("[%lld]",cnt[i]);}printf("\n");
  for(i=1;i<16;i+=2){//if((i&1)==0){continue;}
    if(cnt[i]==0){continue;}
    dt=cnt[i];cnt[i]--;
    pj=dt;
    for(j=0;j<16;j++){if((j&2)==0){continue;}
      if(cnt[j]==0){continue;}
      dt*=cnt[j];cnt[j]--;
      pk=dt;
      for(k=0;k<16;k++){if((k&4)==0){continue;}
        if(cnt[k]==0){continue;}
        dt*=cnt[k];cnt[k]--;
        pl=dt;
        for(l=8;l<16;l++){//if((l&8)==0){continue;}
          if(cnt[l]==0){continue;}
          dt*=cnt[l];cnt[l]--;
          res+=dt;
          cnt[l]++;
          dt=pl;
        }
        cnt[k]++;
        dt=pk;
      }
      cnt[j]++;
      dt=pj;
    }
    cnt[i]++;
  }
  return res;
}

int main(){
  long long n,i,j,k,l,m,f,ce,res=0;
  long long c[512][4];
  long long jt[4],kt[4];
  long long md[4][4];
  long long cnt[16]={0};
  scanf("%lld",&n);
  for(i=0;i<n;i++){
    for(j=0;j<4;j++){
      scanf("%lld",&c[i][j]);
    }
  }
  for(i=0;i<n;i++){
    for(j=0;j<4;j++){jt[j]=c[i][j];}
    for(j=i+1;j<n;j++){
      for(k=0;k<4;k++){
        for(l=0;l<4;l++){
          kt[l]=c[j][(4+k-l)%4];
        }
        ce=1;
        for(l=0;l<4;l++){
          md[l][0]=jt[l];
          md[l][1]=jt[(l+3)%4];
          md[l][2]=kt[(l+3)%4];
          md[l][3]=kt[l];
          ce*=calcce(md[l]);
        }
        for(l=0;l<16;l++){cnt[l]=0;}
        for(l=i+1;l<n;l++){
          if(l==j){continue;}
          f=0;
          for(m=0;m<4;m++){
            if(ispt(c[l],md[m])==1){f|=(1ll<<m);}
          }
          //printf("%lld\n",f);
          cnt[f]++;
        }
        res+=ce*calc(cnt);
      }
    }
  }
  printf("%lld\n",res);
  return 0;
}
