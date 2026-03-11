#include<stdio.h>
 
int q[3];
int ans;
int i;
 
void solve(void){
while(q[0] > 3 && q[1] > 3 && q[2] > 3){
q[0]--;
q[1]--;
q[2]--;
ans++;
}
ans += q[0] / 3;
q[0] %= 3;
ans += q[1] / 3;
q[1] %= 3;
ans += q[2] / 3;
q[2] %= 3;
while(q[0] && q[1] && q[2])q[0]--,q[1]--,--q[2],ans++;
printf("%d\n",ans);
q[0] = q[1] = q[2] = 0;
return;
}
 
int main(void){
for(;;){
for(i=0;i<6;i++)scanf("%d",&ans),q[i % 3]+=ans;
if(q[0] + q[1] + q[2] == 0)break;
ans = 0;
solve();
}
return 0;
}