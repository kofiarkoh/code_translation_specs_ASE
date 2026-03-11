#include<stdio.h>

static inline int getint(){
  int ret = 0;
  int g;
  while((g = getchar() - '0'), g<0);
  ret = g;
  while((g = getchar() - '0'), g>=0){
    ret *= 10;
    ret += g;
  }
  return ret;
}

static inline int cnt(unsigned int bits) {
  bits = (bits & 0x55555555) + (bits >> 1 & 0x55555555);
  bits = (bits & 0x33333333) + (bits >> 2 & 0x33333333);
  bits = (bits & 0x0f0f0f0f) + (bits >> 4 & 0x0f0f0f0f);
  bits = (bits & 0x00ff00ff) + (bits >> 8 & 0x00ff00ff);
  return (bits & 0x0000ffff) + (bits >>16 & 0x0000ffff);
}

int main(){
  int c[20],k,r[20];
  int n,u;
  register unsigned int i,j,l;
  while(n=getint(), u=getint(), n+u){
    int ans = n;

    for(i=0; i<n; i++){
      c[i]=getint(); k=getint();
      r[i] = 0;
      for(j=0; j<k; j++){
	int t = getint();
	r[i] |= (1<<t);
      }
    }

    for(i=(1<<n)-1; i; i--){
      int tni = 0;
      int ok  = 1;
      int cc  = cnt(i);
      if(cc >= ans) continue;

      for(j=0; ok & j<n; j++){
	if(i & (1<<j))
	  tni += c[j];
	if((i & (1<<j)) & ((i & r[j]) ^ r[j]))
	  ok = 0;
      }

      if(tni >= u & ok)
	ans = cc;
    }
    printf("%d\n",ans);
  }
}