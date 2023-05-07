/**
 * @file Bigram.cpp
 * @author Miguel Ángel De La Vega Rodríguez <miguevrod@correo.ugr.es>
 * 
 * @date 2023-03-02
 * 
 */

#include "Solver.h"

Solver::Solver(int size): rows(size),cols(size),
    matrix_coef(new double[rows * cols]), matrix_sol(new double[rows]), matrix_ind(new double[rows])
{
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            matrix_coef[i * cols + j] = 0.0;
        }
    }

    for (int i = 0; i < rows; i++) {
        matrix_sol[i] = 0.0;
        matrix_ind[i] = 0.0;
    }
}

Solver::Solver(const Solver& other): rows(other.rows), cols(other.cols),
    matrix_coef(new double[rows * cols]), matrix_sol(new double[rows]), matrix_ind(new double[rows])
{
    for (int i = 0; i < rows; i++) {
        for (int j = 0; j < cols; j++) {
            matrix_coef[i * cols + j] = other.matrix_coef[i * cols + j];
        }
    }

    for (int i = 0; i < rows; i++) {
        matrix_sol[i] = other.matrix_sol[i];
        matrix_ind[i] = other.matrix_ind[i];
    }
}

Solver::~Solver()
{
    delete[] matrix_coef;
    delete[] matrix_sol;
    delete[] matrix_ind;
}

double* Solver::operator()(int row, int col){
    return &matrix_coef[row * cols + col];
}

void Solver::setMatrixCoef(double matrix_coef[], int size){
    for (int i=0;i<size;i++){
        this->matrix_coef[i] = matrix_coef[i];
    }
}


