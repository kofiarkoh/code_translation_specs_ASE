#include <stdio.h>
#include <math.h>
 
 
int main() {
	int point_num, ans;
	float point[300][2];   
	for(;;){
		scanf("%d\n", &point_num);
		if(point_num == 0)
			break;
		int i;
		float x, y;
		for (i = 0; i < point_num; i++)
			scanf("%f %f\n", &point[i][0], &point[i][1]);
		ans = 0;
		
		for (x = 1; x < 10; x += 0.005) {
			for (y = 1; y < 10; y += 0.005) {
				int count = 0;
				for (i = 0; i < point_num; i++) {
					if (1.0 >=  (x - point[i][0]) * (x - point[i][0]) + (y - point[i][1]) * (y - point[i][1]))
						count++;
				if (count > ans)
					ans = count;
				}
			}
		}
	printf("%d\n", ans);
	}
	return 0;
}