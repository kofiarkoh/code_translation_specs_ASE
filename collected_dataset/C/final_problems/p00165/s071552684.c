#include<stdio.h>
#include<math.h>
int main(){
  int h,i,j,k;
  int o,p,m,n,r,s[78500];
  s[0]=-1000001;
  s[1]=2;
  j=2;
  for(i=3;i<1000000;i+=2){
    r=(int)sqrt(i)+1;
    for(k=3;k<r;k+=2){
      if(i%k==0)
	r=0;
    }
    if(r){
      s[j]=i;
      j++;
    }
  }
  s[j]=2000001;
  for(;;){
    scanf("%d",&n);
    if(n==0)
      break;
    o=0;
    for(h=0;h<n;h++){
      scanf("%d%d",&p,&m);
      for(i=0;i<78499;i++){
	if(s[i]<p-m&&p-m<s[i+1]){
	  i++;
	  break;
	}else if(s[i]==p-m)
	  break;
      }
      for(j=78499;j>0;j--){
	if(s[j-1]<p+m&&p+m<s[j]){
	  j--;
	  break;
	}else if(s[j]==p+m)
	  break;
      }
      if(j-i+1)
	o+=j-i;
      else
	o--;
    }
    printf("%d\n",o);
  }
  return 0;
}