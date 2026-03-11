#include <stdio.h>

int main() {
	char name[101], ans[101], type;
	int n, a;
	
	while(1) {
		scanf("%s %c", name, &type);
		if(type == 'X') { break; }
		
		n = 0;
		a = 0;
		if(type == 'U') {
			while(name[n] != '\0') {
				if(n == 0 && 'a' <= name[0] && name[0] <= 'z') {
					ans[0] = name[0] - 32;
					n ++;
					a ++;
					continue;
				}
				if(name[n] == '_') {
					n ++;
					
					ans[a] = name[n] - 32;
					n ++;
					a ++;
					continue;
				}
				
				ans[a] = name[n];
				a ++;
				n ++;
			}
		}
		if(type == 'L') {
			while(name[n] != '\0') {
				if(n == 0 && 'A' <= name[0] && name[0] <= 'Z') {
					ans[0] = name[0] + 32;
					n ++;
					a ++;
					continue;
				}
				if(name[n] == '_') {
					n ++;
					
					ans[a] = name[n] - 32;
					n ++;
					a ++;
					continue;
				}
				
				ans[a] = name[n];
				a ++;
				n ++;
			}
		}
		if(type == 'D') {
			while(name[n] != '\0') {
				if(n == 0 && 'A' <= name[0] && name[0] <= 'Z') {
					ans[0] = name[0] + 32;
					n ++;
					a ++;
					continue;
				}
				if(n != 0 && 'A' <= name[n] && name[n] <= 'Z') {
					ans[a] = '_';
					a ++;
					
					ans[a] = name[n] + 32;
					a ++;
					n ++;
					continue;
				}
				
				ans[a] = name[n];
				n ++;
				a ++;
			}
		}
		
		printf("%s\n", ans);
	}
	
	return 0;
}