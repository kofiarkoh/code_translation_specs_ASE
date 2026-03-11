#include<stdio.h>

int main()
{
int N, i, a, b, c, max;

scanf("%d", &N);

for(i=0; i<N; i++){
scanf("%d %d %d", &a, &b, &c);

if(a<=b && b<=c){
max = c;
if(c*c==a*a+b*b){
puts("YES");
}else{
puts("NO");
}
}
else if(a<=c && c<=b){
max = b;
if(b*b==a*a+c*c){
puts("YES");
}else{
puts("NO");
}
}
else if(b<=c && c<=a){
max = a;
if(a*a==b*b+c*c){
puts("YES");
}else{
puts("NO");
}
}
else if(b<=a && a<=c){
max = c;
if(c*c==a*a+b*b){
puts("YES");
}else{
puts("NO");
}
}
else if(c<=a && a<=b){
max = b;
if(b*b==a*a+c*c){
puts("YES");
}else{
puts("NO");
}
}
else if(c<=b && b<=a){
max = a;
if(a*a==b*b+c*c){
puts("YES");
}else{
puts("NO");
}
}

}

return 0;
}