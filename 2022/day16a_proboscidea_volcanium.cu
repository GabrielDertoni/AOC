#include <iostream>
#include <cuda.h>

class Matrix {
public:

    __device__ __host__ int& get(int i, int j) {
        return data[i * width + j];
    }

private:
    int *data;
    int width;
    int height;
};

__global__ void compute_permutations(Matrix graph, int *best) {
}

int permutation_flow(Matrix graph, int *flows, int *path, int n) {
    int flow = 0;
    int turns = 0;
    for (int i = 0; i < n && turns < 30; i++) {
        if (i == 0)
            turns += graph.get(0, path[i]) + 1;
        else
            turns += graph.get(path[i - 1], path[i]) + 1;

        flow += flows[path[i]] * (30 - turns);
    }
    return flow;
}

int main() {
    return 0;
}
