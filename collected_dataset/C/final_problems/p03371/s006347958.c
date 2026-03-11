#include <stdio.h>
 
int main(){
int a,b,c,x,y;
int ab = 0;
int i = 0;
scanf("%d %d %d %d %d",&a,&b,&c,&x,&y);
if((a + b) / 2 > c){
	if(x > y){
	for(i;i < y * 2;i++){
	ab += c;
}
	if((a / 2) > c){
	ab += c * (x - y) * 2;
}else{
	ab += a * (x - y);
}
}
	if(y > x){
	for(i;i < x * 2;i++){
	ab += c;
}
	if((b / 2) > c){
	ab += c*(y - x) * 2;
}else{
	ab += b*(y - x);
}
}
printf("%d",ab);
}
else{
	printf("%d\n",a * x + b * y);
}
}