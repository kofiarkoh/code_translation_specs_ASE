
    #include <stdio.h>
int main(){
  int n,t,i;
  long ans=0;
  long timer=0;
  scanf("%d %d",&n,&t);
  long str[n];
  scanf("%ld",&str[0]);

  for(i=0;i<n-1;i++){
    scanf("%ld",&str[i]);
  	if(timer + t >str[i] ){
    	ans=ans+ (long)(str[i]- timer);
      	timer=str[i];
    }else{
      timer=str[i];
      ans+=(long)t;
    }
   
   // printf("%d   ",ans);
  }
   ans+=t;
  printf("%ld\n",ans);
return 0;
}
