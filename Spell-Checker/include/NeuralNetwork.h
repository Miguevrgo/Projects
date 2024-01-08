/**
 * @file NeuralNetwork.h
 * @author Miguel Angel De la Vega Rodríguez
 * @brief NeuralNetwork class header file
 *
 * NeuralNetwork class header file, in this first project, it will be just a first approach to the neural network
 * using a simple one.
 */

#ifndef SPELL_CHECKER_NEURALNETWORK_H
#define SPELL_CHECKER_NEURALNETWORK_H

#include <iostream>
#include <vector>

class Neuron {
public:
    Neuron(int NeuronWeight);
    Neuron(int NeuronWeight, int NeuronBias);
    ~Neuron() = default;
    void transfer();
private:
    size_t NeuronWeight;
    size_t NeuronBias;
    size_t NeuronOutput;

};


class NeuralNetwork {

};


#endif //SPELL_CHECKER_NEURALNETWORK_H
